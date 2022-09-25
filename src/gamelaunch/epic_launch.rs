use subprocess::*;
// use registry::*;
// use std::{collections::HashMap, hash::Hash};
use winreg::*;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
// use std::os::*;
use std::env;
use std::io::BufReader;
// use std::io::prelude::*;
use std::fs::File;
// use std::path::Path;
// use std::ffi::OsStr;
use std::iter::zip;
use std::{thread, time};
// use json::*;
use glob::glob;
use serde_json::{Map, Value, from_reader};
// use psutil::*;
use sysinfo::*;
use regex::*;
use webbrowser::*;


pub fn launch_with_epic_simple(mut ideal_args: Vec<String>) -> Result<Popen> {
    let epic_rl_exe_path = locate_epic_rl_binary();
    // let argv = [epic_rl_exe_path, ideal_args, "-EpicPortal".to_string()];
    let mut argv = [epic_rl_exe_path.clone()].to_vec();
    argv.append(&mut ideal_args);
    argv.push("-EpicPortal".to_string());
    let argv = argv.as_slice();
    // let print = argv.join(" ");
    // println!("{print}");
    let res;
    if epic_rl_exe_path.len() != 0 {
        res = Popen::create(argv, PopenConfig::default())
    } else {
        panic!("Epic simple launch method failed")
    }
    // let final_res = match res {
    //     Ok(res) => res,
    //     Err(error) => panic!("Unable to launch via Epic due to: {error}")
    // };
    return res
}

// pub enum PopenEnum {
//     Popen,
//     IsPopen
// }

// pub fn launch_with_epic_login_trick(ideal_args: Vec<String>) -> Popen_enum {
//     let pid: Pid;
//     let proc_name: String;
//     let cmd_line: Vec<String>;
//     (pid, proc_name, cmd_line) = get_running_processes("RocketLeague.exe".to_string(), vec!["-pipe".to_string()]);
//     if proc_name != "".to_string() {
//         let cmd_line = get_epic_login_trick_args(ideal_args);
//         if cmd_line == "".to_string() {
//             return PopenEnum
//         }
//     }
// }

// fn get_epic_login_trick_args(ideal_args: Vec<String>) -> Vec<String> {
//     open("com.epicgames.launcher://apps/Sugar?action=launch&silent=true").unwrap();
//     let mut pid: Pid = Pid::from(0);
//     let mut proc_name: String = "".to_string();
//     let mut cmd_line: Vec<String> = Vec::<String>::new();
//     for _i in 0..10 {
//         let time = time::Duration::new(1, 0);
//         thread::sleep(time);
//         (pid, proc_name, cmd_line) = get_running_processes("RocketLeague.exe".to_string(), vec!["-EpicPortal".to_string()]);
//         if proc_name != "".to_string() {
//             break
//         }
//     }
//     if proc_name == "".to_string() {
//         return cmd_line
//     }
//     let sys = System::new_all();
//     let proc = sys.process(pid).unwrap();
//     proc.kill();
//     return cmd_line
// }

pub fn locate_epic_rl_binary() -> String {
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

// fn get_running_processes(process_name: String, required_args: Vec<String>) -> (Pid, String, Vec<String>) {
//     let sys = System::new_all();
//     let res = sys.processes_by_name(&process_name);
//     let pid: Pid = Pid::from(0);
//     for process in res {
//         let pid = process.pid();
//         let proc_name = process.name();
//         let cmd = process.cmd();
//         let cmd = &cmd[1..];
//         let mut matching_args: bool = true;
//         for (arg_req, arg) in zip(&required_args, cmd) {
//             let reg = Regex::new(&arg_req.to_lowercase()).unwrap();
//             matching_args = reg.is_match(&arg.to_lowercase());
//             if !matching_args {
//                 break
//             }
//         }
//         if matching_args {
//             return (pid, proc_name.to_owned(), cmd.to_owned())
//         }
//     }
//     return (pid, "".to_owned(), Vec::<String>::new())
// }