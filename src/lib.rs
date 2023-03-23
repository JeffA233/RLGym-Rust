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

use std::{collections::HashMap, thread::{JoinHandle, self}, time::Duration, iter::zip, path::PathBuf};
use crossbeam_channel::{bounded, Sender, Receiver};
use ndarray::Dim;
use std::path::Path;
use numpy::PyArray;
use itertools::izip;
use std::io::{BufWriter, Write};
use std::io::ErrorKind::PermissionDenied;
use std::fs::OpenOptions;

use crate::gym::Gym;
use pyo3::prelude::*;
// use rayon::prelude::*;

use obs_builders::{aspo4_array_2::AdvancedObsPadderStacker2, obs_builder::ObsBuilder};
use reward_functions::custom_rewards::{get_custom_reward_func, get_custom_reward_func_mult_inst};
use action_parsers::{necto_parser_2::NectoAction, continous_act::ContinuousAction};
// use action_parsers::discrete_act::DiscreteAction;
use conditionals::custom_conditions::CombinedTerminalConditions;
use state_setters::custom_state_setters::custom_state_setters;
use windows::Win32::Foundation::CloseHandle;


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
    pub fn new(team_size: i32, tick_skip: usize, seed: Option<u64>) -> Self {
    let term_cond = Box::new(CombinedTerminalConditions::new(tick_skip));
    let reward_fn = get_custom_reward_func();
    let obs_build = Box::new(AdvancedObsPadderStacker2::new(None, Some(1)));
    let mut obs_build_vec = Vec::<Box<(dyn ObsBuilder + Send + 'static)>>::new();
    obs_build_vec.push(obs_build);
    let act_parse = Box::new(NectoAction::new());
    let state_set = Box::new(custom_state_setters(team_size, seed));
    let gym = make::make(
        Some(100000.), 
        Some(tick_skip), 
        Some(true), 
        Some(team_size as usize), 
        None, 
        None,
        None,  
        term_cond, 
        reward_fn, 
        obs_build_vec, 
        act_parse, 
        state_set, 
        None, 
        true, 
        false, 
        false, 
        true);
        GymWrapper { gym: gym }
    }

    pub fn reset(&mut self, seed: Option<u64>) -> PyResult<Vec<Vec<f64>>> {
        let obs = self.gym.reset(Some(false), seed);
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
    pub fn new(team_size: i32, gravity: f64, boost: f64, self_play: bool, tick_skip: usize, pipe_name: Option<usize>, sender: Sender<Vec<f64>>) -> Self {
    let term_cond = Box::new(CombinedTerminalConditions::new(tick_skip));
    let reward_fn = get_custom_reward_func_mult_inst(sender);
    let mut obs_build_vec = Vec::<Box<dyn ObsBuilder + Send>>::new();
    for i in 0..team_size*2 {
        obs_build_vec.push(Box::new(AdvancedObsPadderStacker2::new(None, Some(1))));
    }
    // let obs_build = Box::new(AdvancedObsPadderStacker::new(None, Some(5)));
    let act_parse = Box::new(NectoAction::new());
    let state_set = Box::new(custom_state_setters(team_size, None));
    let gym = make::make(Some(100000.), 
        Some(tick_skip), 
        Some(self_play), 
        Some(team_size as usize), 
        Some(gravity), 
        Some(boost),
        pipe_name, 
        term_cond, 
        reward_fn, 
        obs_build_vec, 
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
        let obs = self.gym.reset(Some(false), None);
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
    sends: Vec<Sender<ManagerPacket>>,
    recvs: Vec<Receiver<WorkerPacket>>,
    n_agents_per_env: Vec<i32>,
    total_agents: usize
}

/// packet that comes from the manager
pub enum ManagerPacket {
    Step {actions: Vec<Vec<f64>>},
    Reset,
    Close
}

/// packet that comes from the worker
pub enum WorkerPacket {
    StepRet {obs: Vec<Vec<f64>>, reward: Vec<f64>, done: bool, info: HashMap<String, f64>},
    StepRetDone {obs: Vec<Vec<f64>>, reward: Vec<f64>, done: bool, info: HashMap<String, f64>, terminal_obs: Vec<Vec<f64>>},
    ResetRet {obs: Vec<Vec<f64>>},
    InitReturn
}

#[pymethods]
impl GymManager {
    #[new]
    pub fn new(match_nums: Vec<i32>, gravity_nums: Vec<f64>, boost_nums: Vec<f64>, self_plays: Vec<bool>, tick_skip: usize) -> Self {
        let mut recv_vec = Vec::<Receiver<WorkerPacket>>::new();
        let mut send_vec = Vec::<Sender<ManagerPacket>>::new();
        let mut thrd_vec = Vec::<JoinHandle<()>>::new();
        let mut curr_id = 0;

        let (reward_send, reward_recv) = bounded(20000);
        let reward_file_loc = r"F:\Users\Jeffrey\AppData\Local\Temp";
        let reward_file_name = format!(r"{}\rewards.txt", reward_file_loc);
        let reward_path = Path::new(&reward_file_name).to_owned();
        // let reward_thrd = thread::spawn(move || file_put_worker(reward_recv, reward_path));
        thread::spawn(move || file_put_worker(reward_recv, reward_path));

        // redo agent numbers for self-play case, need to redo to just be agents on one team instead of for whole match
        let mut corrected_match_nums = Vec::<i32>::new();

        for (match_num, self_play) in match_nums.iter().zip(self_plays.iter()) {
            if *self_play {
                corrected_match_nums.push(*match_num);
            } else {
                corrected_match_nums.push(*match_num/2);
            }
        }

        for (match_num, gravity, boost, self_play) in izip!(match_nums.clone(), gravity_nums.clone(), boost_nums.clone(), self_plays.clone()) {
            let mut retry_loop = true;
            // try to loop until the game successfully launches
            while retry_loop {
                let reward_send_local = reward_send.clone();
                let send_local: Sender<ManagerPacket>;
                let rx: Receiver<ManagerPacket>;
                let tx: Sender<WorkerPacket>;
                let recv_local: Receiver<WorkerPacket>;
                (send_local, rx) = bounded(1);
                (tx, recv_local) = bounded(1);
                let thrd1 = thread::spawn(move || worker(match_num/2, gravity, boost, self_play, tick_skip, tx, rx, curr_id as usize, reward_send_local));
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
            n_agents_per_env: corrected_match_nums.clone(),
            total_agents: corrected_match_nums.iter().sum::<i32>() as usize
        }
    }
    
    pub fn reset(&self, py: Python<'_>) -> PyResult<Py<PyArray<f64, Dim<[usize; 2]>>>> {
        for sender in &self.sends {
            sender.send(ManagerPacket::Reset).unwrap();
        }
        // self.sends.par_iter().for_each(|send| send.send(ManagerPacket::Reset).unwrap());

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
        let flat_obs_numpy = PyArray::from_vec2(py, &flat_obs).unwrap().to_owned();
        // return Ok(flat_obs)
        return Ok(flat_obs_numpy)
    }

    pub fn step_async(&mut self, actions: Vec<Vec<f64>>) -> PyResult<()> {
        let mut i: usize = 0;
        for (sender, agent_num) in zip(&self.sends, &self.n_agents_per_env) {
            let acts = actions[i..i+*agent_num as usize].to_vec();
            sender.send(ManagerPacket::Step { actions: acts }).unwrap();
            i += *agent_num as usize
        }
        self.waiting = true;
        Ok(())
    }

    pub fn step_wait(&mut self, py: Python<'_>) -> PyResult<(Py<PyArray<f64, Dim<[usize; 2]>>>, Vec<f64>, Vec<bool>, Vec<HashMap<String, f64>>, Vec<Option<Vec<Vec<f64>>>>)> {
        let mut flat_obs = Vec::<Vec<f64>>::with_capacity(self.total_agents);
        let mut flat_rewards = Vec::<f64>::with_capacity(self.total_agents);
        let mut flat_dones = Vec::<bool>::with_capacity(self.total_agents);
        let mut flat_infos = Vec::<HashMap<String,f64>>::with_capacity(self.total_agents);
        let mut flat_term_obs = Vec::<Option<Vec<Vec<f64>>>>::with_capacity(self.total_agents);

        for (receiver, n_agents) in zip(&self.recvs, &self.n_agents_per_env) {
            let data = receiver.recv().unwrap();

            let (obs, rew, done, info, terminal_obs) = match data {
                WorkerPacket::StepRet { obs, reward, done, info } => (obs, reward, done, info, None),
                WorkerPacket::StepRetDone { obs, reward, done, info, terminal_obs } => (obs, reward, done, info, Some(terminal_obs)),
                _ => panic!("StepRet was not returned from Step command given")
            };
            // same as above in reset fn and for rewards it will be a vec of f64 to be "flat" and so on
            flat_obs.extend(obs);

            flat_rewards.extend(rew);

            // since PyO3 cannot currently use HashMaps with enums we must push this outside of Rust into Python
            match terminal_obs {
                Some(obs) => flat_term_obs.extend(vec![Some(obs); *n_agents as usize]),
                None => flat_term_obs.extend(vec![None; *n_agents as usize])
            };
            // since the env will emit done and info as the same for every agent in the match, we just multiply them to fill the number of agents
            flat_dones.extend(vec![done; *n_agents as usize]);
            flat_infos.extend(vec![info; *n_agents as usize]);
        }
        let flat_obs_numpy = PyArray::from_vec2(py, &flat_obs).unwrap().to_owned();
        self.waiting = false;
        return Ok((flat_obs_numpy, flat_rewards, flat_dones, flat_infos, flat_term_obs));
    }

    pub fn close(&mut self) -> PyResult<()> {
        for sender in &self.sends {
            sender.send(ManagerPacket::Close).unwrap();
        }
        Ok(())
    }
}

pub fn worker(team_num: i32, gravity: f64, boost: f64, self_play: bool, tick_skip: usize, send_chan: Sender<WorkerPacket>, rec_chan: Receiver<ManagerPacket>, pipe_name: usize, reward_sender: Sender<Vec<f64>>) {
    // launches env and then sends the reset action to a new thread since receiving a message from the plugin will be blocking,
    // waits for x seconds for thread to return the env if it is a success else tries to force close the pipe and 
    // make the gym crash (which should terminate the game)
    let mut env = GymWrapperRust::new(team_num, gravity, boost, self_play, tick_skip, Some(pipe_name), reward_sender);
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
        // trying to force close pipe to remove the blocking call to the pipe in the gym
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
        let mut obs: Vec<Vec<f64>>;
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
                let out;
                // check if match is done, unfortunately we must send this to Python somehow because HashMaps with enums 
                // (for multiple-type HashMaps) cannot be translated into Python
                if done {
                    let terminal_obs = obs;
                    obs = env.reset();
                    out = send_chan.send(WorkerPacket::StepRetDone {obs, reward, done, info, terminal_obs});
                } else {
                    out = send_chan.send(WorkerPacket::StepRet {obs, reward, done, info});
                }
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

fn file_put_worker(receiver: Receiver<Vec<f64>>, reward_file: PathBuf) {
    loop {
        let out = OpenOptions::new().append(true).read(true).open(reward_file.as_path());

        let file = match out {
            Err(out) => {
                println!("file error: {out}");
                // half a second 
                thread::sleep(Duration::new(0, 500000000));
                if out.kind() == PermissionDenied {continue} else {continue};},
            Ok(_file) => _file
        };
        let mut buf = BufWriter::new(&file);

        let mut i = 0;
        loop {
            let recv_data = receiver.recv();
            let returns_local = match recv_data {
                Ok(data) => data,
                Err(err) => {
                    println!("recv err in file_put_worker: {err}");
                    break;
                }
            };

            let mut string = String::new();
            string = string + "[";
            for i in 0..returns_local.len()-1 {
                string = string + &format!("{}, ", returns_local[i])
            }
            string = string + &format!("{}]", returns_local[returns_local.len()-1]);
            writeln!(&mut buf, "{}", string).unwrap();
            
            i += 1;

            if receiver.is_empty() || i > 1000 {
                let out = buf.flush();
                match out {
                    Ok(out) => out,
                    Err(err) => println!("buf.flush in logger failed with error: {err}")
                };
                i = 0;
                // break;
            }
        }
        println!("logger worker is exiting");
        break;
    }
}

