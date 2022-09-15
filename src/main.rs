mod action_parsers;
mod common_values;
mod communication;
mod conditionals;
mod envs;
pub mod gamelaunch;
pub mod gamestates;
mod math;
mod obs_builders;
mod reward_functions;
mod state_setters;
use std::env::*;
use std::path::Path;

// math.norm_func();

fn main() {
    // println!("Hello, world!");
    // let line = gamelaunch::epic_launch::locate_epic_RL_binary();
    let cur_dir = current_dir().unwrap();
    let full_path = cur_dir.join(Path::new("RLMultiInjector.exe"));
    let cur_dur_display = full_path.display();
    println!("{cur_dur_display}");
    // let gamest = gamestates::physics_object::PhysicsObject::default();
    // let pos = gamest.position;
    // let pos_str = pos.iter().map(|x| "{x}").collect::<String>();
    // println!("{pos_str}")
}
