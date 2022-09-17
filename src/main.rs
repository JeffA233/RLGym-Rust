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
// use std::env::*;
// use std::path::Path;
use std::{thread, time};

use gamelaunch::launch;

// math.norm_func();

fn main() {
    // println!("Hello, world!");
    // let line = gamelaunch::epic_launch::locate_epic_RL_binary();
    // let cur_dir = current_dir().unwrap();
    // let full_path = cur_dir.join(Path::new("RLMultiInjector.exe"));
    // let cur_dur_display = full_path.display();
    // println!("{cur_dur_display}");
    let pipe_id = communication::communication_handler::format_pipe_id(0);
    let launch_type = launch::LaunchPreference::new();
    let res = launch::launch_rocket_league(pipe_id, launch_type.EPIC);
    let _res = match res {
        Ok(popen) => popen,
        Err(error) => panic!("error from launch function: {error}")
    };
    let dur = time::Duration::new(3, 0);
    thread::sleep(dur);
    gamelaunch::launch::run_injector();

    // let gamest = gamestates::physics_object::PhysicsObject::default();
    // let pos = gamest.position;
    // let pos_str = pos.iter().map(|x| "{x}").collect::<String>();
    // println!("{pos_str}")
}
