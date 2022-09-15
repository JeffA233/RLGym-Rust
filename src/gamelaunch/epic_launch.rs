use subprocess::*;
// use registry::*;
// use std::{collections::HashMap, hash::Hash};
use winreg::*;
use winreg::enums::*;
// use std::os::*;
use std::env;
use std::io::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
// use json::*;
use glob::{glob, glob_with};
use serde_json::*;
use psutil::*;



pub fn launch_with_epic_simple(ideal_args: Vec<String>) {
    let epic_rl_exe_path = locate_epic_RL_binary();
    if epic_rl_exe_path.len() != 0 {
        // Popen{

        // }
    }
}

pub fn locate_epic_RL_binary() -> String {
    let possible_registry_locations_regkey = [
        RegKey::predef(HKEY_LOCAL_MACHINE), 
        RegKey::predef(HKEY_LOCAL_MACHINE),
        RegKey::predef(HKEY_CURRENT_USER),
        RegKey::predef(HKEY_CURRENT_USER)];

    let possible_registry_locations_loc = [
        "SOFTWARE\\Epic Games\\EpicGamesLauncher",
        "SOFTWARE\\WOW6432Node\\Epic Games\\EpicGamesLauncher",
        "SOFTWARE\\Epic Games\\EpicGamesLauncher",
        "SOFTWARE\\WOW6432Node\\Epic Games\\EpicGamesLauncher"
    ];

    fn search_for_manifest_file(app_data_path: String) -> Map<String, Value> {
        // let mut data = HashMap::<String, String>::new();
        let mut data = Map::new();
        let app_str = &(app_data_path.clone() + "*.item");
        let glob_res = glob(app_str);
        let glob_values = match glob_res {
            Ok(files) => files,
            Err(error) => return data
        };
        // println!("{app_str}");
        for entry in glob_values {

            let entry_line = match entry {
                Ok(path) => path,
                Err(error) => continue
            };
            // let entry_display = entry_line.display();
            // println!("{entry_display}");
            let file_result = File::open(&entry_line);
            let file = match file_result {
                Ok(file) => file,
                Err(error) => continue
            };
            let buf_file = BufReader::new(file);
            // let data: HashMap<String, String> = from_reader(buf_file);
            let reader_data = from_reader::<BufReader<File>, Map<String, Value>>(buf_file);
            data = match reader_data {
                Ok(data) => data,
                Err(error) => {println!("{error}"); continue}
            };
            let data_option = data.get("MandatoryAppFolderName");
            let data_name = match data_option {
                Some(data) => data,
                None => continue
            };
            if data_name == "rocketleague" {
                break
            }
        };
        
        return data
    }

    let mut binary_data = Map::new();

    let mut install_path = String::new();

    for i in 0..4 {
        let query_val = RegKey::open_subkey(&possible_registry_locations_regkey[i], &possible_registry_locations_loc[i]);
        let query_val = match query_val {
            Ok(path) => path,
            Err(error) => continue
        };
        let path_res = query_val.get_value::<String, &str>("AppDataPath");
        let path = match path_res {
            Ok(path) => path,
            Err(error) => continue
        };

        let full_path = path + r"Manifests\";
        // println!("{full_path}");
        binary_data = search_for_manifest_file(full_path);

        // let mut install_path = String::new();
        let res = binary_data.get("InstallLocation");
        let is_value = match res {
            Some(res) => true,
            None => false
        };
        if is_value {
            let value = binary_data.get("InstallLocation").unwrap().as_str().unwrap().to_owned();
            // let value_actual = value.as_str().unwrap().to_owned();
            let value_2 = binary_data.get("LaunchExecutable").unwrap().as_str().unwrap().to_owned();
            // let value_2_actual = value_2.as_str().unwrap().to_owned();
            install_path = value + r"\" + &value_2;
            // println!("{install_path}");
            break
        } else {
            continue
        }   
    }

    // println!("{install_path}");

    // let mut install_path = String::new();
    if install_path.len() == 0 {
        let starter_path = env::var("programdata");
        let path = match starter_path {
            Ok(path) => path,
            Err(error) => panic!("{error}")
        };
        let full_path = path + r"Epic\" + r"EpicGamesLauncher\" + r"Data\" + r"Manifests\";

        binary_data = search_for_manifest_file(full_path);

        if binary_data.get("InstallLocation") != None {
            let value = binary_data.get("InstallLocation").unwrap().as_str().unwrap().to_owned();
            let value_2 = binary_data.get("LaunchExecutable").unwrap().as_str().unwrap().to_owned();
            install_path = value + "/" + &value_2;
        }
        return install_path
    }
    return install_path
}

fn get_running_processes(process_name: String, required_args: Vec<String>) {
    let matching_processes: Vec<String> = Vec::<String>::new();
}