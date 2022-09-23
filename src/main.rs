// use ndarray::*;

use action_parsers::necto_parser_2::NectoAction;
use conditionals::custom_conditions::CombinedTerminalConditions;
use obs_builders::aspo4_array::AdvancedObsPadderStacker;
use reward_functions::custom_rewards::{SB3CombinedLogReward, get_custom_reward_func};
use state_setters::random_state::RandomState;
use crate::state_setters::state_setter::StateSetter;

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
// use std::io::{BufWriter, Write};
// use std::env::*;
// use std::path::Path;
// use std::{thread, time};

// use gamelaunch::launch;

// math.norm_func();

fn main() {
    // let str = format!("{:02x}", 8 as u8);
    // println!("{str}");
    let term_cond = Box::new(CombinedTerminalConditions::new(8));
    let reward_fn = get_custom_reward_func();
    let obs_build = Box::new(AdvancedObsPadderStacker::new(None, None));
    let act_parse = Box::new(NectoAction::new());
    let state_set = Box::new(RandomState::new(None, None, None));
    let actions = vec![vec![55.]];
    let mut gym = make::make(None, 
        None, 
        Some(false), 
        Some(1), 
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
        false);
    for i in 0..100 {
        gym.step(actions.clone());
    }
    gym.reset(None);
    for i in 0..100 {
        gym.step(actions.clone());
    }
}
