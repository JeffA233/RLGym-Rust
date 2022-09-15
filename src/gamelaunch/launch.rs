use std::path::Path;
use std::env::*;
use subprocess::*;

pub struct ROCKET_LEAGUE_PROCESS_INFO {
    GAMEID: usize,
    PROGRAM_NAME: String,
    PROGRAM: String,
    REQUIRED_ARGS: String
}

impl ROCKET_LEAGUE_PROCESS_INFO {
    fn new() -> Self {
        ROCKET_LEAGUE_PROCESS_INFO {
            GAMEID: 252950,
            PROGRAM_NAME: "RocketLeague.exe".to_string(),
            PROGRAM: "RocketLeague.exe".to_string(),
            REQUIRED_ARGS: "-pipe".to_string()
        }
    }
    fn get_ideal_args(pipe_id: usize) -> [String; 3] {
        return ["-pipe".to_string(), pipe_id.to_string(), "-nomovie".to_string()]
    }
}

pub struct LaunchPreference {
    STEAM: String,
    EPIC: String,
    EPIC_LOGIN_TRICK: String
}

impl LaunchPreference {
    fn new() -> Self {
        LaunchPreference {
            STEAM: "steam".to_string(),
            EPIC: "epic".to_string(),
            EPIC_LOGIN_TRICK: "_login_trick".to_string()
        }
    }
}

fn run_injector() {
    println!("Executing injector...");
    let cur_dir = current_dir().unwrap();
    // let plugin_path = Path::new("plugin");
    let injector_path = cur_dir.join(Path::new("RLMultiInjector.exe"));
    Popen::create(&[injector_path], PopenConfig::default()).unwrap();
}

fn launch_rocket_league(pipe_id: usize, launch_preference: String) -> Result<Popen> {
    let ideal_args = ROCKET_LEAGUE_PROCESS_INFO::get_ideal_args(pipe_id);
    let default_launch_pref = LaunchPreference::new();
    if !launch_preference.starts_with(&default_launch_pref.EPIC) || !launch_preference.starts_with(&default_launch_pref.STEAM) {
        if Path::new(&launch_preference).exists() {
            return Popen::create(&[launch_preference], PopenConfig::default())
        } else {
            println!("path_to_rl doesn't point to RocketLeague.exe")
        }
    }
    if launch_preference.starts_with(&default_launch_pref.EPIC) {
        if launch_preference == default_launch_pref.EPIC_LOGIN_TRICK {
            let proc = launch_with_epic_login_trick(ideal_args);
            
        }
    }
}