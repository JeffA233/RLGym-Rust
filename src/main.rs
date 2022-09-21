// use ndarray::*;

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
use std::fs::File;
use std::fs::*;
use std::io::{BufWriter, Write};
// use std::env::*;
// use std::path::Path;
// use std::{thread, time};

// use gamelaunch::launch;

// math.norm_func();

fn main() {
    let out = File::open("rewards.txt");
    
    match out {
        Ok(out) => remove_file("rewards.txt").unwrap(),
        Err(error) => ()
    };

    let out = File::create("rewards.txt").unwrap();
    let mut ret = BufWriter::new(out);
    let vec = vec![1., 2.5, 3.5, 4.5, 6.6];
    let mut string = String::new();
    string = string + "[";
    for i in 0..vec.len()-2 {
        string = string + &format!("{}, ", vec[i])
    }
    string = string + &format!("{}]", vec[vec.len()-1]);
    writeln!(&mut ret, "{}", string).unwrap();
    // writeln!(&mut ret, "{}", string).unwrap();
}
