// use ndarray::*;
use std::{time::*, thread::JoinHandle};

// use std::collections::HashMap;

use action_parsers::necto_parser_2::NectoAction;
use communication::communication_handler::f32vec_as_u8_slice;
use conditionals::custom_conditions::CombinedTerminalConditions;
use envs::game_match::GameMatch;
use gamestates::game_state::{GameState};
use obs_builders::aspo4_array::AdvancedObsPadderStacker;
use reward_functions::custom_rewards::get_custom_reward_func;
// use state_setters::random_state::RandomState;

use crate::{state_setters::{custom_state_setters::custom_state_setters, default_state::{DefaultState, DefaultStateTester}}, obs_builders::obs_builder::ObsBuilder, action_parsers::action_parser::ActionParser, conditionals::common_conditions::TimeoutCondition, reward_functions::default_reward::RewardFn};
// use crate::state_setters::state_setter::StateSetter;

pub mod random_test;
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
// use std::fs::File;
// use std::fs::*;
use std::io::{BufWriter, Write, stdin};
// use std::env::*;
// use std::path::Path;
use std::{thread, time};
use crossbeam_channel::{bounded, unbounded, Sender, Receiver};

// use gamelaunch::launch;

// math.norm_func();

fn main() {
    // let v = vec![93.0];
    // let bytes = f32vec_as_u8_slice(&v);
    let obs = vec![vec![93, 93, 93], vec![92, 93, 93], vec![91, 93, 93]];
    let mut vec = Vec::<Vec<i32>>::new();
    vec.extend(obs);
    // let term_cond = Box::new(CombinedTerminalConditions::new(1));
    let term_cond = Box::new(TimeoutCondition::new(225));
    let reward_fn = get_custom_reward_func();
    // let obs_build = Box::new(AdvancedObsPadderStacker::new(None, Some(5)));
    let mut act_parse = Box::new(NectoAction::new());
    // let state_set = Box::new(custom_state_setters(1));
    let state_set = Box::new(DefaultStateTester::new());
    let actions = vec![vec![55.]];
    // let mut gym = make::make(Some(100000.), 
    //     Some(0), 
    //     Some(false), 
    //     Some(1), 
    //     None, 
    //     None,
    //     Some(1), 
    //     term_cond, 
    //     reward_fn, 
    //     obs_build, 
    //     act_parse, 
    //     state_set, 
    //     None, 
    //     true, 
    //     false, 
    //     false, 
    //     false);

    // --TESTING OF MATCH/REWARDS/ETC.--
    // let match_ = gym._game_match;
    // let mut match_ = GameMatch::new(reward_fn, 
    //     term_cond, 
    //     obs_build, 
    //     act_parse, 
    //     state_set, 
    //     Some(2),
    //     Some(8),
    //     Some(100.),
    //     Some(1.),
    //     Some(1.),
    //     Some(false));

    // --TESTING OF OBS--
    let fake_state = GameState::new_test();

    let mut obs_thread_vec = Vec::<Sender<Manager>>::new();
    let (send, recv_local) = unbounded();
    for i in 0..6 {
        let fake_state_clone = fake_state.clone();
        let send_nonlocal = send.clone();
        let (send, recv) = unbounded();
        let obs_build = Box::new(AdvancedObsPadderStacker::new(None, Some(5)));
        let reward_fn = get_custom_reward_func();
        obs_thread_vec.push(send);

        thread::spawn(move || {
            worker(send_nonlocal, recv, fake_state_clone, obs_build, reward_fn);
        });
    }

    let start_time = Instant::now();
    for i in 0..100000 {
        for thrd in &obs_thread_vec {
            thrd.send(Manager::Step);
        }
        
        for x in 0..obs_thread_vec.len() {
            recv_local.recv();
        }
    }
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed with threads: {seconds_elapsed}");
    let fps = (120.*360.)/seconds_elapsed;
    println!("fps: {fps}");

    let mut obs_build = Box::new(AdvancedObsPadderStacker::new(None, Some(5)));
    let mut reward_fn = get_custom_reward_func();

    let start_time = Instant::now();
    let mut obs;
    // let mut rew;
    for i in 0..100000 {       
        for x in 0..obs_thread_vec.len() {
            obs = obs_build.build_obs(&fake_state.players[0], &fake_state, &vec![0., 0., 0., 0., 0., 0., 0., 0.]);
            // rew = reward_fn.get_reward(&fake_state.players[0], &fake_state, &vec![0., 0., 0., 0., 0., 0., 0., 0.]);
        }
    }
    let duration = start_time.elapsed();
    let seconds_elapsed = duration.as_secs_f64();
    println!("seconds elapsed with no threads: {seconds_elapsed}");
    let fps = (120.*360.)/seconds_elapsed;
    println!("fps: {fps}");

    // seconds elapsed with threads: 12.8681321
    // seconds elapsed with no threads: 13.5231119
    
    // obs only
    // seconds elapsed with threads: 3.6690481999999998
    // seconds elapsed with no threads: 4.141068

    pub enum Manager {
        Step
    }
    
    /// packet that comes from the worker
    pub enum Worker {
        StepRet {obs: Vec<f64>, rew: f64}
    }

    pub fn worker(send_chan: Sender<Worker>, rec_chan: Receiver<Manager>, fake_state: GameState, mut obs_builder: Box<AdvancedObsPadderStacker>, mut reward_fn: Box<dyn RewardFn + Send>) {
        loop {
            // simple loop that tries to recv for as long as the Manager channel is not hung up waiting for commands from the Manager
            let obs: Vec<f64>;
            // let rew: f64;
            let recv_data = rec_chan.recv();
            let cmd = match recv_data {
                Ok(out) => out,
                Err(err) => {
                    println!("recv err in worker: {err}"); 
                    break;
                }
            };
            match cmd {
                Manager::Step => {
                    obs = obs_builder.build_obs(&fake_state.players[0], &fake_state, &vec![0., 0., 0., 0., 0., 0., 0., 0.]);
                    // rew = reward_fn.get_reward(&fake_state.players[0], &fake_state, &vec![0., 0., 0., 0., 0., 0., 0., 0.]);
                    send_chan.send(Worker::StepRet { obs, rew: 0. }).unwrap();
                }
            };
        }
    }



    // let obs = match_.build_observations(&mut fake_state);
    // let mut out;
    // out = act_parse.parse_actions(vec![vec![43., 50.]], &fake_state);
    // for i in 0..89 {
    //     let act_vec: Vec<Vec<f32>> = vec![vec![i as f32; 2]];
    //     out = act_parse.parse_actions(act_vec, &fake_state);
    // }
    // match_.episode_reset(&fake_state);
    // let obs = match_.build_observations(&mut fake_state);
    // let rew_f32: f32 = rew.iter().sum();
    // println!("{rew_f32}");
    // --END--
    // gym.reset(None, None);
    // gym.step(actions.clone());

    // let mut rew_val: f64 = 0.;
    // let start_time = Instant::now();
    // for _i in 0..(120 * 360) {
    //     let (_obs, reward, done, _info) = gym.step(actions.clone());
    //     if done {
    //         gym.reset(None, None);
    //     }
    //     rew_val += reward[0];
    // }
    // let duration = start_time.elapsed();
    // let seconds_elapsed = duration.as_secs_f64();
    // println!("seconds elapsed: {seconds_elapsed}");
    // let fps = (120.*360.)/seconds_elapsed;
    // println!("fps: {fps}");

    // gym.reset(None);

    // let mut rew_val: f32 = 0.;
    // for _i in 0..(15 * 360) {
    //     let (_obs, reward, done, _info) = gym.step(actions.clone());
    //     if done {
    //         gym.reset(None);
    //     }
    //     let rew_str: String = reward.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    //     // println!("{rew_str}");
    //     rew_val += reward[0];
    // }
    // let end_val = rew_val / (15.*360.);
    // println!("rough reward per tick: {end_val}");
    // println!("closing Rocket League");
    // gym.close();
    // println!("waiting");
    // stdin().read_line(&mut String::new()).unwrap();
}
