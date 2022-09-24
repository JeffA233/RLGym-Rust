// use ndarray::*;
// use std::time::*;

use std::collections::HashMap;

use action_parsers::necto_parser_2::NectoAction;
use conditionals::custom_conditions::CombinedTerminalConditions;
use obs_builders::aspo4_array::AdvancedObsPadderStacker;
use reward_functions::custom_rewards::get_custom_reward_func;
use state_setters::random_state::RandomState;
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
// use std::io::{BufWriter, Write};
// use std::env::*;
// use std::path::Path;
// use std::{thread, time};

// use gamelaunch::launch;

// math.norm_func();

fn main() {
    // let str = format!("{:02x}", 8 as u8);
    // let mut hashmap = HashMap::new();
    // hashmap.insert(0, vec![0, 1, 2]);
    // let temp_vec = vec![0, 1, 2];
    // let out = hashmap.get(&0);
    // let out = match out {
    //     Some(out) => out,
    //     None => &temp_vec
    // };
    // println!("{str}");
    // let vec_1 = vec![5.; 10];
    // let vec_2 = vec![2.; 10];
    // let out = math::element_add_vec(&vec_1, &vec_2);
    // let printable = out.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    // println!("{printable}");
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
    gym.reset(None);
    // let start_time = Instant::now();
    for _i in 0..(20 * 360) {
        let (_obs, _reward, done, _info) = gym.step(actions.clone());
        if done {
            gym.reset(None);
        }
    }
    // let end_time = start_time.elapsed();
    // let seconds_elapsed = end_time.as_secs_f64();
    // println!("seconds elapsed: {seconds_elapsed}");
    // let fps = (120.*360.)/seconds_elapsed;
    // println!("fps: {fps}");
    gym.reset(None);
    let mut rew_val: f32 = 0.;
    for _i in 0..(15 * 360) {
        let (_obs, reward, done, _info) = gym.step(actions.clone());
        if done {
            gym.reset(None);
        }
        rew_val += reward[0];
    }
    let end_val = rew_val / (15.*360.);
    println!("rough reward per tick: {end_val}");
    println!("closing Rocket League");
    gym.close();
}
