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

fn run_injector() {
    println!("Executing injector...");
    let cur_dir = current_dir().unwrap();
    // let plugin_path = Path::new("plugin");
    let injector_path = cur_dir.join(Path::new("RLMultiInjector.exe"));
    Popen::create(&[injector_path], PopenConfig::default()).unwrap();
}