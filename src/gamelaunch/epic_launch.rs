use subprocess::*;
// use registry::*;
use std::{collections::HashMap, hash::Hash};
use winreg::*;
use winreg::enums::*;
// use std::os::*;
use std::io::*;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
// use json::*;
use glob::{glob, glob_with};
use serde_json::*;



pub fn launch_with_epic_simple(ideal_args: Vec<String>) {
    let epic_rl_exe_path = locate_epic_RL_binary();

}

pub fn locate_epic_RL_binary() {
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

    fn search_for_manifest_file(app_data_path: &str) -> HashMap<String, String> {
        let mut data = HashMap::<String, String>::new();
        for entry in glob(app_data_path) {
            let file_result = File::open(app_data_path);
            let file = match file_result {
                Ok(file) => file,
                Err(error) => continue
            };
            let buf_file = BufReader::new(file);
            // let data: HashMap<String, String> = from_reader(buf_file);
            let reader_data = from_reader::<BufReader<File>, HashMap<String, String>>(buf_file);
            data = match reader_data {
                Ok(data) => data,
                Err(error) => continue
            };
            break
        };
        return data
    }

    for i in 0..4 {
        let query_val = RegKey::open_subkey(&possible_registry_locations_regkey[i], &possible_registry_locations_loc[i]).unwrap();
        query_val.query_info();
    }
}