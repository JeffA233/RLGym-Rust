use std::path::Path;
use std::env::*;
use subprocess::*;
use crate::gamelaunch::epic_launch::launch_with_epic_simple;

pub struct RocketLeagueProcessInfo {
    // game_id: usize,
    // program_name: String,
    // program: String,
    // required_args: String
}

impl RocketLeagueProcessInfo {
    // fn new() -> Self {
    //     RocketLeagueProcessInfo {
    //         game_id: 252950,
    //         program_name: "RocketLeague.exe".to_string(),
    //         program: "RocketLeague.exe".to_string(),
    //         required_args: "-pipe".to_string()
    //     }
    // }
    pub fn get_ideal_args(pipe_id: String) -> [String; 3] {
        return ["-pipe".to_string(), pipe_id, "-nomovie".to_string()]
    }
}

pub struct LaunchPreference {
    pub steam: String,
    pub epic: String,
    pub epic_login_trick: String
}

impl LaunchPreference {
    pub fn new() -> Self {
        LaunchPreference {
            steam: "steam".to_string(),
            epic: "epic".to_string(),
            epic_login_trick: "_login_trick".to_string()
        }
    }
}

pub fn run_injector() {
    println!("Executing injector...");
    let cur_dir = current_dir().unwrap();
    // let plugin_path = Path::new("plugin");
    let injector_path = cur_dir.join(Path::new("RLMultiInjector.exe"));
    Popen::create(&[injector_path], PopenConfig::default()).unwrap();
}

pub fn launch_rocket_league(pipe_id: String, launch_preference: &String) -> Popen {
    let ideal_args = RocketLeagueProcessInfo::get_ideal_args(pipe_id);
    let default_launch_pref = LaunchPreference::new();
    if !(launch_preference == &default_launch_pref.epic || launch_preference == &default_launch_pref.steam) {
        if Path::new(&launch_preference).exists() {
            return Popen::create(&[launch_preference], PopenConfig::default()).unwrap()
        } else {
            println!("path_to_rl doesn't point to RocketLeague.exe")
        }
    }
    // if launch_preference.starts_with(&default_launch_pref.EPIC) {
        // if launch_preference == default_launch_pref.EPIC_LOGIN_TRICK {
        //     let proc = launch_with_epic_login_trick(ideal_args);
            
        // }
    let game_process = launch_with_epic_simple(ideal_args.to_vec());
    let game_process = match game_process {
        Ok(proc) => proc,
        Err(error) => panic!("Could not start Rocket League with epic simple: {error}")
    };
    println!("Launched Epic version");
    return game_process
}