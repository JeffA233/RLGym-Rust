pub mod action_parsers;
pub mod common_values;
pub mod communication;
pub mod conditionals;
pub mod envs;
pub mod gamelaunch;
pub mod gamestates;
pub mod math;
pub mod obs_builders;
pub mod reward_functions;
pub mod state_setters;
pub mod gym;
pub mod make;

use std::{collections::HashMap, thread::{JoinHandle, self}, sync::mpsc::{Receiver, sync_channel, SyncSender}, time::Duration, iter::zip};

use crate::gym::Gym;
use pyo3::prelude::*;

use obs_builders::aspo4_array::AdvancedObsPadderStacker;
use reward_functions::{custom_rewards::get_custom_reward_func};
use action_parsers::necto_parser_2::NectoAction;
use conditionals::{custom_conditions::CombinedTerminalConditions};
use state_setters::custom_state_setters::custom_state_setters;
use windows::Win32::Foundation::{CloseHandle};


/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

/// A Python module implemented in Rust.
#[pymodule]
pub fn rlgym_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<GymWrapper>()?;
    m.add_class::<GymManager>()?;
    Ok(())
}

/// Wrapper for the gym to be used in Python as a module
#[pyclass]
pub struct GymWrapper {
    gym: Gym
}

#[pymethods]
impl GymWrapper {
    #[new]
    /// create the gym wrapper to be used (team_size: i32, tick_skip: usize)
    pub fn new(team_size: i32, tick_skip: usize) -> Self {
    let term_cond = Box::new(CombinedTerminalConditions::new(tick_skip));
    let reward_fn = get_custom_reward_func();
    let obs_build = Box::new(AdvancedObsPadderStacker::new(None, Some(5)));
    let act_parse = Box::new(NectoAction::new());
    let state_set = Box::new(custom_state_setters(team_size));
    let gym = make::make(Some(100000.), 
        Some(tick_skip), 
        Some(true), 
        Some(team_size as usize), 
        None, 
        None,
        None,  
        term_cond, 
        reward_fn, 
        obs_build, 
        act_parse, 
        state_set, 
        None, 
        true, 
        false, 
        false, 
        true);
        GymWrapper { gym: gym }
    }

    pub fn reset(&mut self) -> PyResult<Vec<Vec<f64>>> {
        let obs = self.gym.reset(Some(false));
        Ok(obs)
    }

    pub fn step(&mut self, actions: Vec<Vec<f64>>) -> PyResult<(Vec<Vec<f64>>, Vec<f64>, bool, HashMap<String, f64>)> {
        let obs;
        let reward;
        let done;
        let info;
        (obs, reward, done, info) = self.gym.step(actions);
        return Ok((obs, reward, done, info))
    }

    pub fn close(&mut self) -> PyResult<()> {
        self.gym.close();
        Ok(())
    }
}
// end of Python RLGym env
// -------------------------------------------------------------------------------------
// start of RLGym multiple-instance manager for Python

pub struct GymWrapperRust {
    gym: Gym
}

impl GymWrapperRust {
    /// create the gym wrapper to be used (team_size: i32, tick_skip: usize)
    pub fn new(team_size: i32, tick_skip: usize, pipe_name: Option<usize>) -> Self {
    let term_cond = Box::new(CombinedTerminalConditions::new(tick_skip));
    let reward_fn = get_custom_reward_func();
    let obs_build = Box::new(AdvancedObsPadderStacker::new(None, Some(5)));
    let act_parse = Box::new(NectoAction::new());
    let state_set = Box::new(custom_state_setters(team_size));
    let gym = make::make(Some(100000.), 
        Some(tick_skip), 
        Some(true), 
        Some(team_size as usize), 
        None, 
        None,
        pipe_name, 
        term_cond, 
        reward_fn, 
        obs_build, 
        act_parse, 
        state_set, 
        None, 
        true, 
        false, 
        false, 
        true);
        GymWrapperRust { gym: gym }
    }

    pub fn reset(&mut self) -> Vec<Vec<f64>> {
        let obs = self.gym.reset(Some(false));
        return obs;
    }

    pub fn step(&mut self, actions: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<f64>, bool, HashMap<String, f64>) {
        let obs;
        let reward;
        let done;
        let info;
        (obs, reward, done, info) = self.gym.step(actions);
        return (obs, reward, done, info);
    }

    pub fn close(&mut self) {
        self.gym.close();
    }
}

// RLGym wrapper for Rust (mostly used to preset the gym options)
// -------------------------------------------------------------------------------------
// RLGym Manager in Rust for Python interface

/// manager for multiple instances of the gym done in Rust for Python use
#[pyclass]
pub struct GymManager {
    #[pyo3(get)]
    waiting: bool,
    // threads: Vec<JoinHandle<()>>,
    sends: Vec<SyncSender<ManagerPacket>>,
    recvs: Vec<Receiver<WorkerPacket>>,
    n_agents_per_env: Vec<i32>
}

/// packet that comes from the manager
pub enum ManagerPacket {
    Step {actions: Vec<Vec<f64>>},
    Reset,
    Close
    // StepRet {obs: Vec<Vec<f64>>, reward: Vec<f64>, done: bool, info: HashMap<String, f64>},
    // ResetRet {obs: Vec<Vec<f64>>}
}

/// packet that comes from the worker
pub enum WorkerPacket {
    StepRet {obs: Vec<Vec<f64>>, reward: Vec<f64>, done: bool, info: HashMap<String, f64>},
    ResetRet {obs: Vec<Vec<f64>>},
    InitReturn
}

#[pymethods]
impl GymManager {
    #[new]
    pub fn new(match_nums: Vec<i32>, tick_skip: usize) -> Self {
        let mut recv_vec = Vec::<Receiver<WorkerPacket>>::new();
        let mut send_vec = Vec::<SyncSender<ManagerPacket>>::new();
        let mut thrd_vec = Vec::<JoinHandle<()>>::new();
        let mut curr_id = 0;

        for match_num in match_nums.clone() {
            let mut retry_loop = true;
            // try to loop until the game successfully launches
            while retry_loop {
                let send_local: SyncSender<ManagerPacket>;
                let rx: Receiver<ManagerPacket>;
                let tx: SyncSender<WorkerPacket>;
                let recv_local: Receiver<WorkerPacket>;
                (send_local, rx) = sync_channel(0);
                (tx, recv_local) = sync_channel(0);
                let thrd1 = thread::spawn(move || worker(match_num/2, tick_skip, tx, rx, curr_id as usize));
                curr_id += 1;

                // wait for worker to send back a packet or if it never does then restart loop to try again
                let out = recv_local.recv_timeout(Duration::new(60, 0));

                match out {
                    Ok(packet) => packet,
                    Err(err) => {
                        println!("recv timed out in new func: {err}");
                        continue;
                    }
                };

                // gather all of the local channels and threads for later use (if game launches are successful)
                recv_vec.push(recv_local);
                send_vec.push(send_local);
                thrd_vec.push(thrd1);
                retry_loop = false;
            }
        }

        GymManager {
            waiting: false,
            // threads: thrd_vec,
            sends: send_vec,
            recvs: recv_vec,
            n_agents_per_env: match_nums
        }
    }
    
    pub fn reset(&self) -> PyResult<Vec<Vec<f64>>> {
        for sender in &self.sends {
            sender.send(ManagerPacket::Reset).unwrap();
        }

        // flat obs means that the obs should be of shape [num_envs, obs_size] (except this is a Vec so it's not a "shape" but the length)
        let mut flat_obs = Vec::<Vec<f64>>::new();
        for receiver in &self.recvs{
            let data = receiver.recv().unwrap();
            let obs = match data {
                WorkerPacket::ResetRet { obs } => obs,
                _ => panic!("ResetRet was not returned from Reset command given")
            };
            for internal_vec in obs {
                flat_obs.push(internal_vec);
            }
        }
        return Ok(flat_obs)
    }

    pub fn step_async(&mut self, actions: Vec<Vec<f64>>) -> PyResult<()> {
        // let mut i: usize = 0;
        for (sender, action) in zip(&self.sends, actions) {
            let acts = vec![action];
            sender.send(ManagerPacket::Step { actions: acts }).unwrap();
            // i += 1;
        }
        self.waiting = true;
        Ok(())
    }

    pub fn step_wait(&mut self) -> PyResult<(Vec<Vec<f64>>, Vec<f64>, Vec<bool>, Vec<HashMap<String, f64>>)> {
        let mut flat_obs = Vec::<Vec<f64>>::new();
        let mut flat_rewards = Vec::<f64>::new();
        let mut flat_dones = Vec::<bool>::new();
        let mut flat_infos = Vec::<HashMap<String,f64>>::new();

        for (receiver, n_agents) in zip(&self.recvs, &self.n_agents_per_env) {
            let data = receiver.recv().unwrap();

            let (obs, rew, done, info) = match data {
                WorkerPacket::StepRet { obs, reward, done, info } => (obs, reward, done, info),
                _ => panic!("StepRet was not returned from Step command given")
            };
            // same as above in reset and for rewards it will be a vec of f64 to be "flat" and so on
            for internal_vec in obs {
                flat_obs.push(internal_vec);
            }
            for internal_rew in rew {
                flat_rewards.push(internal_rew);
            }
            // since the env will emit done and info as the same for every agent in the match, we just multiply them to fill the number of agents
            flat_dones.append(&mut vec![done; *n_agents as usize]);
            flat_infos.append(&mut vec![info; *n_agents as usize]);
        }
        self.waiting = false;
        return Ok((flat_obs, flat_rewards, flat_dones, flat_infos));
    }

    pub fn close(&mut self) -> PyResult<()> {
        for sender in &self.sends {
            sender.send(ManagerPacket::Close).unwrap();
        }
        Ok(())
    }
}

pub fn worker(team_num: i32, tick_skip: usize, send_chan: SyncSender<WorkerPacket>, rec_chan: Receiver<ManagerPacket>, pipe_name: usize) {
    // launches env and then sends the reset action to a new thread since receiving a message from the plugin will be blocking,
    // waits for x seconds for thread to return the env if it is a success else tries to force close the pipe and 
    // make the gym crash (which should terminate the game)
    let mut env = GymWrapperRust::new(team_num, tick_skip, Some(pipe_name));
    let pipe_id = env.gym._comm_handler._pipe;
    let thrd = thread::spawn(move || {env.reset(); return env;});
    let mut i = 0;
    while !thrd.is_finished() {
        thread::sleep(Duration::new(1, 0));
        i += 1;
        if i > 70 {
            break;
        }
    }
    if i > 70 {
        unsafe {
            CloseHandle(pipe_id);
        }
        println!("successfully closed pipe handle in worker");
        return;
    }
    env = thrd.join().unwrap();
    send_chan.send(WorkerPacket::InitReturn).unwrap();

    // for cmd in rec_chan.iter() {
    loop {
        // simple loop that tries to recv for as long as the Manager channel is not hung up waiting for commands from the Manager
        let obs: Vec<Vec<f64>>;
        let reward: Vec<f64>;
        let done: bool;
        let info: HashMap<String, f64>;
        let recv_data = rec_chan.recv();
        let cmd = match recv_data {
            Ok(out) => out,
            Err(err) => {
                println!("recv err in worker: {err}"); 
                break;
            }
        };
        match cmd {
            ManagerPacket::Step { actions } => {
                (obs, reward, done, info) = env.step(actions);
                let out = send_chan.send(WorkerPacket::StepRet {obs, reward, done, info});
                match out {
                    Ok(res) => res,
                    Err(err) => {
                        println!("send err in worker: {err}"); 
                        break;
                    }
                }
            }
            ManagerPacket::Close => {break}
            ManagerPacket::Reset => {
                obs = env.reset();
                // send_chan.send(WorkerPacket::ResetRet { obs }).unwrap();
                let out = send_chan.send(WorkerPacket::ResetRet { obs });
                match out {
                    Ok(res) => res,
                    Err(err) => {
                        println!("send err in worker: {err}"); 
                        break;
                    }
                }
            }
        };
    }
    env.close();
}
