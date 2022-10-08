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

use std::{collections::HashMap, thread::{JoinHandle, self}, sync::mpsc::{Receiver, Sender, channel}, time::Duration, iter::zip, process::id};

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

    pub fn close(&mut self) {
        self.gym.close();
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
        obs
    }

    pub fn step(&mut self, actions: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<f64>, bool, HashMap<String, f64>) {
        let obs;
        let reward;
        let done;
        let info;
        (obs, reward, done, info) = self.gym.step(actions);
        return (obs, reward, done, info)
    }

    pub fn close(&mut self) {
        self.gym.close();
    }
}

// RLGym wrapper for Rust (mostly used to preset the gym options)
// -------------------------------------------------------------------------------------
// RLGym Manager in Rust for Python interface

#[pyclass]
pub struct GymManager {
    waiting: bool,
    // threads: Vec<JoinHandle<()>>,
    sends: Vec<Sender<ManagerPacket>>,
    recvs: Vec<Receiver<WorkerPacket>>,
    n_agents_per_env: Vec<i32>
}

pub enum ManagerPacket {
    Step {actions: Vec<Vec<f64>>},
    Reset,
    Close
    // StepRet {obs: Vec<Vec<f64>>, reward: Vec<f64>, done: bool, info: HashMap<String, f64>},
    // ResetRet {obs: Vec<Vec<f64>>}
}

pub enum WorkerPacket {
    StepRet {obs: Vec<Vec<f64>>, reward: Vec<f64>, done: bool, info: HashMap<String, f64>},
    ResetRet {obs: Vec<Vec<f64>>}
}

#[pymethods]
impl GymManager {
    #[new]
    pub fn new(match_nums: Vec<i32>, tick_skip: usize) -> Self {
        let mut send_local: Sender<ManagerPacket>;
        let mut rx: Receiver<ManagerPacket>;
        let mut tx: Sender<WorkerPacket>;
        let mut recv_local: Receiver<WorkerPacket>;
        let mut recv_vec = Vec::<Receiver<WorkerPacket>>::new();
        let mut send_vec = Vec::<Sender<ManagerPacket>>::new();
        let mut thrd_vec = Vec::<JoinHandle<()>>::new();
        let mut curr_id = 0;

        for match_num in match_nums.clone() {
            let mut retry_loop = true;
            while retry_loop {
                (send_local, rx) = channel();
                (tx, recv_local) = channel();
                let thrd1 = thread::spawn(move || worker(match_num/2, tick_skip, tx, rx, curr_id as usize));
                curr_id += 1;
                let err = send_local.send(ManagerPacket::Reset);
                match err {
                    Ok(out) => out,
                    Err(err) => {println!("tx send error: {err}"); continue;}
                }
                let out = recv_local.recv_timeout(Duration::new(60, 0));

                // let out = thread::spawn(move || try_remote(recv_local));
                
                // while !out.is_finished() {
                //     thread::sleep(Duration::new(1, 0));
                // }
                // let ret = out.join().unwrap();

                match out {
                    Ok(packet) => packet,
                    Err(err) => {
                        println!("recv timed out in new func: {err}");
                        continue;
                    }
                };

                // recv_local = match ret {
                //     (ManagerPacket::Error, Receiver) => {continue},
                //     (ManagerPacket::ResetRet {obs}, Receiver) => {Receiver},
                //     other => {continue}
                //     // (ManagerPacket::Send {data}, Receiver) => {continue},
                //     // (ManagerPacket::Close, Receiver) => {continue},
                //     // (ManagerPacket::StepRet { obs, reward, done, info }, Receiver) => Receiver,
                //     // (ManagerPacket::Reset, Receiver) => {continue}
                // };

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

    pub fn reset(&self) -> Vec<Vec<f64>> {
        for sender in &self.sends {
            sender.send(ManagerPacket::Reset).unwrap();
        }

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
        return flat_obs
    }

    pub fn step_async(&mut self, actions: Vec<Vec<f64>>) {
        let mut i: usize = 0;
        for (sender, n_agents) in zip(&self.sends, &self.n_agents_per_env) {
            let acts = actions[i..i+*n_agents as usize].to_vec();
            sender.send(ManagerPacket::Step { actions: acts }).unwrap();
            i += *n_agents as usize;
        }
        self.waiting = true;
    }

    pub fn step_wait(&mut self) -> (Vec<Vec<f64>>, Vec<f64>, Vec<bool>, Vec<HashMap<String, f64>>) {
        let mut flat_obs = Vec::<Vec<f64>>::new();
        let mut flat_rewards = Vec::<f64>::new();
        let mut flat_dones = Vec::<bool>::new();
        let mut flat_infos = Vec::<HashMap<String,f64>>::new();

        for (receiver, n_agents) in zip(&self.recvs, &self.n_agents_per_env) {
            let data = receiver.recv().unwrap();

            let (obs, rew, done, info) = match data {
                WorkerPacket::StepRet { obs, reward, done, info } => (obs, reward, done, info),
                _ => panic!("StepRet was not returned from Reset command given")
            };
            for internal_vec in obs {
                flat_obs.push(internal_vec);
            }
            for internal_rew in rew {
                flat_rewards.push(internal_rew);
            }
            flat_dones.append(&mut vec![done; *n_agents as usize]);
            flat_infos.append(&mut vec![info; *n_agents as usize]);
        }
        self.waiting = false;
        return (flat_obs, flat_rewards, flat_dones, flat_infos);
    }

    pub fn close(&mut self) {
        for sender in &self.sends {
            sender.send(ManagerPacket::Close).unwrap();
        }
    }
}

pub fn worker(team_num: i32, tick_skip: usize, send_chan: Sender<WorkerPacket>, rec_chan: Receiver<ManagerPacket>, pipe_name: usize) {
    // maybe launch on separate thread and check if the gym is responding or not to check if Rocket League successfully launched
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
        println!("successfully closed handle in worker");
        return;
    }
    env = thrd.join().unwrap();
    // let mut obs: Vec<Vec<f64>>;
    // let mut reward: Vec<f64>;
    // let mut done: bool;
    // let mut info: HashMap<&str, f64>;
    // let ref mut gym = env;
    

    for cmd in rec_chan.iter() {
        let obs: Vec<Vec<f64>>;
        let reward: Vec<f64>;
        let done: bool;
        let info: HashMap<String, f64>;
        // let ref mut gym = env;
        // let recv_data = rec_chan.recv();
        // let cmd = match recv_data {
        //     Some(out) => out,
        //     None => {println!("sender hung up"); env.close(); break;}
        // };
        match cmd {
            ManagerPacket::Step { actions } => {
                (obs, reward, done, info) = env.step(actions);
                send_chan.send(WorkerPacket::StepRet {obs, reward, done, info}).unwrap();
            }
            ManagerPacket::Close => {break}
            ManagerPacket::Reset => {
                obs = env.reset();
                send_chan.send(WorkerPacket::ResetRet { obs }).unwrap();
            }
            // ManagerPacket::Error => {break}
            // _ => panic!("Action other than Step, Close or Reset given in worker")
        };
        // drop(env)
        // if matched_cmd == 1 {
        //     (obs, reward, done, info) = env.step(data.unwrap());
        // } else if matched_cmd == 2 {
        //     env.close();
        //     break;
        // }
        // let packet = ManagerPacket::StepRet { obs, reward, done, info };
        // let (obs, reward, done, info) = out;
        // match cmd {
        //     ManagerPacket::Step { data: _ } => {
        //         send_chan.send(ManagerPacket::StepRet {obs, reward, done, info});
        //     }
        // }
    };
    env.close();
}

// pub fn try_remote(rec: Receiver<ManagerPacket>) -> (ManagerPacket, Receiver<ManagerPacket>) {
//     let out = rec.recv_timeout(Duration::new(60, 0));
//     let out = match out {
//         Ok(out) => out,
//         Err(err) => {println!("tx send error in try_remote: {err}"); ManagerPacket::Error}
//     };
//     return (out, rec);
// }

// pub struct GymManager {
//     waiting: bool,
//     threads: Vec<JoinHandle<()>>,
//     sends: Vec<Sender<(Vec<Vec<f64>>, Vec<f64>, bool, HashMap<&str, f64>)>>,
//     recvs: Vec<Receiver<(Vec<Vec<f64>>, Vec<f64>, bool, HashMap<&str, f64>)>>,
//     n_agents_per_env: Vec<i32>
// }

// pub enum ManagerPacket {
//     Step {data: Vec<Vec<f64>>},
//     Reset,
//     Close,
//     StepRet {obs: Vec<Vec<f64>>, reward: Vec<f64>, done: bool, info: HashMap<&'static str, f64>},
//     ResetRet {obs: Vec<Vec<f64>>},
//     Error
// }

// impl GymManager {
//     pub fn new(team_nums: Vec<i32>, tick_skip: usize) -> Self {
//         let mut send_local: Sender<(i32, Option<Vec<f64>>)>;
//         let mut recv_remote: Receiver<(i32, Option<Vec<f64>>)>;
//         let mut send_remote: Sender<(Vec<Vec<f64>>, Vec<f64>, bool, HashMap<&str, f64>)>;
//         let mut recv_local: Receiver<(Vec<Vec<f64>>, Vec<f64>, bool, HashMap<&str, f64>)>;
//         let mut recv_vec = Vec::<Receiver<(Vec<Vec<f64>>, Vec<f64>, bool, HashMap<&str, f64>)>>::new();
//         let mut send_vec = Vec::<Sender<(i32, Option<Vec<f64>>)>>::new();
//         let mut thrd_vec = Vec::<JoinHandle<()>>::new();

//         for team_num in team_nums.clone() {
//             let mut retry_loop = true;
//             while retry_loop {
//                 (send_local, recv_remote) = channel();
//                 (send_remote, recv_local) = channel();
//                 let thrd1 = thread::spawn(move || worker(team_num, tick_skip, send_remote, recv_remote));
//                 let err = send_local.send((3, None));
//                 match err {
//                     Ok(out) => out,
//                     Err(err) => {println!("tx send error: {err}"); thread::sleep(Duration::new(1, 0)); continue;}
//                 }
//                 let out = thread::spawn(move || try_remote(recv_local));
                
//                 while !out.is_finished() {
//                     thread::sleep(Duration::new(1, 0));
//                 }
//                 let ret = out.join().unwrap();

//                 recv_local = match ret {
//                     (ManagerPacket::Error, Receiver) => {continue},
//                     (ManagerPacket::ResetRet {obs}, Receiver) => {Receiver},
//                     other => {continue}
//                     // (ManagerPacket::Send {data}, Receiver) => {continue},
//                     // (ManagerPacket::Close, Receiver) => {continue},
//                     // (ManagerPacket::StepRet { obs, reward, done, info }, Receiver) => Receiver,
//                     // (ManagerPacket::Reset, Receiver) => {continue}
//                 };

//                 recv_vec.push(recv_local);
//                 send_vec.push(send_local);
//                 thrd_vec.push(thrd1);
//                 retry_loop = false;
//             }
//         }

//         GymManager {
//             waiting: false,
//             threads: thrd_vec,
//             sends: send_vec,
//             recvs: recv_vec,
//             n_agents_per_env: team_nums.iter().map(|x| x*2).collect()
//         }
//     }
// }

// pub fn worker(team_num: i32, tick_skip: usize, send_chan: Sender<ManagerPacket>, rec_chan: Receiver<ManagerPacket>) {
//     let mut env = GymWrapperRust::new(team_num, tick_skip);
//     // let mut obs: Vec<Vec<f64>>;
//     // let mut reward: Vec<f64>;
//     // let mut done: bool;
//     // let mut info: HashMap<&str, f64>;
//     // let ref mut gym = env;

//     loop {
//         let mut obs: Vec<Vec<f64>> = Vec::<Vec<f64>>::new();
//         let mut reward: Vec<f64>;
//         let mut done: bool;
//         let mut info: HashMap<&str, f64>;
//         // let ref mut gym = env;
//         let recv_data = rec_chan.recv();
//         let cmd = match recv_data {
//             Ok(out) => out,
//             Err(err) => {println!("worker recv error: {err}"); env.close(); break;}
//         };
//         let (matched_cmd, data) = match cmd {
//             ManagerPacket::Step { data } => {
//                 // env.step(data)
//                 // ManagerPacket::StepRet { obs, reward, done, info };
//                 (1, Some(data))
//             }
//             // ManagerPacket::Close => {break env;}
//             ManagerPacket::Close => (2, None),
//             ManagerPacket::Reset => (3, None),
//             // ManagerPacket::Reset => {
//             //     (env.reset(), Vec::<f64>::new(), false, HashMap::<&str, f64>::new())
//             //     // ManagerPacket::ResetRet { obs };
//             // }
//             // ManagerPacket::Error => {break env;}
//             // _ => {break env;}
//             // _ => (obs, reward, done, info)
//             _ => (0, None)
//         };
//         // drop(env)
//         if matched_cmd == 1 {
//             (obs, reward, done, info) = env.step(data.unwrap());
//         } else if matched_cmd == 2 {
//             env.close();
//             break;
//         }
//         // let packet = ManagerPacket::StepRet { obs, reward, done, info };
//         // let (obs, reward, done, info) = out;
//         // match cmd {
//         //     ManagerPacket::Step { data: _ } => {
//         //         send_chan.send(ManagerPacket::StepRet {obs, reward, done, info});
//         //     }
//         // }
//     };
//     env.close();
// }

// pub fn try_remote(rec: Receiver<ManagerPacket>) -> (ManagerPacket, Receiver<ManagerPacket>) {
//     let out = rec.recv_timeout(Duration::new(60, 0));
//     let out = match out {
//         Ok(out) => out,
//         Err(err) => {println!("tx send error in try_remote: {err}"); ManagerPacket::Error}
//     };
//     return (out, rec);
// }