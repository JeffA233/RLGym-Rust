// use ndarray::*;
use std::time::*;

// use std::collections::HashMap;

use action_parsers::necto_parser_2::NectoAction;
use conditionals::custom_conditions::CombinedTerminalConditions;
use envs::game_match::GameMatch;
use gamestates::game_state::{GameState};
use obs_builders::aspo4_array::AdvancedObsPadderStacker;
use reward_functions::custom_rewards::get_custom_reward_func;
// use state_setters::random_state::RandomState;

use crate::state_setters::custom_state_setters::custom_state_setters;
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
// use std::{thread, time};

// use gamelaunch::launch;

// math.norm_func();

fn main() {
    let term_cond = Box::new(CombinedTerminalConditions::new(1));
    let reward_fn = get_custom_reward_func();
    let obs_build = Box::new(AdvancedObsPadderStacker::new(None, None));
    let act_parse = Box::new(NectoAction::new());
    let state_set = Box::new(custom_state_setters(1));
    // let actions = vec![vec![55.]];
    // let mut gym = make::make(Some(100000.), 
    //     Some(0), 
    //     Some(true), 
    //     Some(1), 
    //     None, 
    //     None, 
    //     term_cond, 
    //     reward_fn, 
    //     obs_build, 
    //     act_parse, 
    //     state_set, 
    //     None, 
    //     true, 
    //     false, 
    //     false, 
    //     true);
    // --TESTING OF MATCH/REWARDS/ETC.--
    // let match_ = gym._game_match;
    let mut match_ = GameMatch::new(reward_fn, 
        term_cond, 
        obs_build, 
        act_parse, 
        state_set, 
        Some(1),
        Some(8),
        Some(100.),
        Some(1.),
        Some(1.),
        Some(false));
    let mut fake_state = GameState::new_test();
    let obs = match_.build_observations(&mut fake_state);
    // let rew_f32: f32 = rew.iter().sum();
    // println!("{rew_f32}");
    // --END--
    // gym.reset(None);
    // gym.step(actions.clone());

    // let mut rew_val: f32 = 0.;
    // let start_time = Instant::now();
    // for _i in 0..(120 * 360) {
    //     let (_obs, reward, done, _info) = gym.step(actions.clone());
    //     if done {
    //         gym.reset(None);
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
    println!("waiting");
    stdin().read_line(&mut String::new()).unwrap();
}
