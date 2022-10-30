use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, math::{element_add_vec, element_mult_vec}, common_values::BLUE_TEAM};

use super::{common_rewards::{player_ball_rewards::VelocityPlayerToBallReward, ball_goal_rewards::VelocityBallToGoalReward, misc_rewards::{SaveBoostReward, VelocityReward, EventReward}}, default_reward::RewardFn};

// use numpy::*;
// use ndarray::*;
use std::{fs::*, path::PathBuf, collections::HashMap};
use crossbeam_channel::Sender;
use std::io::{BufWriter, Write};
use std::io::ErrorKind::*;
// use std::fs::File;
use fs2::FileExt;
use std::thread;
use std::path::Path;
// use rayon::prelude::*;


/// returns configured custom rewards for Matrix usage, this part is meant only for the non-Rust multi-instance configuration
pub fn get_custom_reward_func() -> Box<dyn RewardFn + Send> {
    let mut reward_fn_vec = Vec::<Box<dyn RewardFn + Send>>::new();

    reward_fn_vec.push(Box::new(VelocityPlayerToBallReward::new(None)));
    reward_fn_vec.push(Box::new(VelocityBallToGoalReward::new(None, None)));
    reward_fn_vec.push(Box::new(GatherBoostReward::new()));
    reward_fn_vec.push(Box::new(SaveBoostReward::new()));
    reward_fn_vec.push(Box::new(LeftKickoffReward::new()));
    reward_fn_vec.push(Box::new(JumpTouchReward::new(Some(150.), None)));
    reward_fn_vec.push(Box::new(VelocityReward::new(None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, None, None, None, Some(5.), Some(45.), Some(25.), None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, Some(100.), None, None, None, None, None, None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, None, Some(-100.), None, None, None, None, None)));
    reward_fn_vec.push(Box::new(JumpReward::new()));
    reward_fn_vec.push(Box::new(DribbleAirTouchReward::new(Some(180.), None, None, Some(0.8))));

    // let weights = vec![0.03, 0.2, 5.0, 0.01, 0.7, 2.0, 0.02, 1.0, 1.0, 1.0, 0.006];
    let weights = vec![0.03, 0.2, 10.0, 0.06, 0.7, 3.0, 0.03, 1.0, 1.0, 1.0, 0.006, 6.0];
    assert!(weights.len() == reward_fn_vec.len());

    Box::new(SB3CombinedLogReward::new(
        reward_fn_vec, 
        weights,
        Some(r"F:\Users\Jeffrey\AppData\Local\Temp".to_string()),
        Some(0.1)
    ))
}

/// returns configured custom rewards for Matrix usage, built for Rust multi-instance 
pub fn get_custom_reward_func_mult_inst(reward_send_chan: Sender<Vec<f64>>) -> Box<dyn RewardFn + Send> {
    let mut reward_fn_vec = Vec::<Box<dyn RewardFn + Send>>::new();

    reward_fn_vec.push(Box::new(VelocityPlayerToBallReward::new(None)));
    reward_fn_vec.push(Box::new(VelocityBallToGoalReward::new(None, None)));
    reward_fn_vec.push(Box::new(GatherBoostReward::new()));
    reward_fn_vec.push(Box::new(SaveBoostReward::new()));
    reward_fn_vec.push(Box::new(LeftKickoffReward::new()));
    reward_fn_vec.push(Box::new(JumpTouchReward::new(Some(150.), None)));
    reward_fn_vec.push(Box::new(VelocityReward::new(None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, None, None, None, Some(5.), Some(45.), Some(25.), None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, Some(100.), None, None, None, None, None, None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, None, Some(-100.), None, None, None, None, None)));
    reward_fn_vec.push(Box::new(JumpReward::new()));
    reward_fn_vec.push(Box::new(DribbleAirTouchReward::new(Some(180.), None, None, Some(0.8))));

    // let weights = vec![0.03, 0.2, 5.0, 0.01, 0.7, 2.0, 0.02, 1.0, 1.0, 1.0, 0.006];
    let weights = vec![0.03, 0.2, 10.0, 0.06, 0.7, 3.0, 0.03, 1.0, 1.0, 1.0, 0.006, 6.0];
    assert!(weights.len() == reward_fn_vec.len());

    Box::new(SB3CombinedLogRewardMultInst::new(
        reward_fn_vec, 
        weights,
        Some(r"F:\Users\Jeffrey\AppData\Local\Temp".to_string()),
        Some(0.1),
        reward_send_chan
    ))
}

/// reward for having jump available for use
pub struct JumpReward {}

impl JumpReward {
    pub fn new() -> Self {
        JumpReward {}
    }
}

impl RewardFn for JumpReward {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, _state: &GameState, _previous_action: &Vec<f64>) -> f64 {
        return player.has_jump as i32 as f64
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}

/// reward only for the agent that is on the left of their respective side (meant to try to be the normal "left goes for kickoff" rule)
pub struct LeftKickoffReward {
    vel_dir_reward: VelocityPlayerToBallReward,
    kickoff_id_orange: i32,
    kickoff_id_blue: i32
}

impl LeftKickoffReward {
    pub fn new() -> Self {
        LeftKickoffReward {
            vel_dir_reward: VelocityPlayerToBallReward::new(None),
            kickoff_id_orange: -1,
            kickoff_id_blue: -1
        }
    }
}

impl RewardFn for LeftKickoffReward {
    fn reset(&mut self, _initial_state: &GameState) {
        self.vel_dir_reward.reset(_initial_state);
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        if state.ball.position.x == 0. && state.ball.position.y == 0. {
            if self.kickoff_id_blue == -1 || self.kickoff_id_orange == -1 {
                let mut blue_car = &state.players[0];
                let mut orange_car = &state.players[1];
                self.kickoff_id_blue = -1;
                self.kickoff_id_orange = -1;

                for car in &state.players {
                    // find a car on each team to compare against
                    let mut blue_car_found = false;
                    let mut orange_car_found = false;
                    if car.team_num == BLUE_TEAM && !blue_car_found {
                        blue_car = car;
                        blue_car_found = true;
                    } else if !orange_car_found {
                        orange_car = car;
                        orange_car_found = true;
                    }
                    if blue_car_found && orange_car_found {
                        break;
                    }
                }

                for car in &state.players {
                    if car.team_num == blue_car.team_num {
                        if car.car_data.position.y >= blue_car.car_data.position.y &&
                        car.car_data.position.x > blue_car.car_data.position.x {
                            blue_car = car;
                        }
                    }
                    if car.team_num == orange_car.team_num {
                        if car.inverted_car_data.position.y >= orange_car.inverted_car_data.position.y &&
                        car.inverted_car_data.position.x > orange_car.inverted_car_data.position.x {
                            orange_car = car;
                        }
                    }
                }
                self.kickoff_id_blue = blue_car.car_id;
                self.kickoff_id_orange = orange_car.car_id;
            }

            if player.team_num == BLUE_TEAM {
                if player.car_id == self.kickoff_id_blue {
                    return self.vel_dir_reward.get_reward(player, state, previous_action)
                } else {
                    return 0.
                }
            } else {
                if player.car_id == self.kickoff_id_orange {
                    return self.vel_dir_reward.get_reward(player, state, previous_action)
                } else {
                    return 0.
                }
            }

        } else {
            self.kickoff_id_blue = -1;
            self.kickoff_id_orange = -1;
            return 0.
        }
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}

/// reward for touching the ball in the air above the specified min_height, then taken to the exponent
pub struct JumpTouchReward {
    min_height: f64,
    exp: f64
}

impl JumpTouchReward {
    fn new(min_height: Option<f64>, exp: Option<f64>) -> Self {
        let min_height = match min_height {
            Some(min_height) => min_height,
            None => 93.
        };
        let exp = match exp {
            Some(exp) => exp,
            None => 0.2
        };

        JumpTouchReward {
            min_height: min_height,
            exp: exp
        }
    }
}

impl RewardFn for JumpTouchReward {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, _previous_action: &Vec<f64>) -> f64 {
        if player.ball_touched && !player.on_ground && state.ball.position.z >= self.min_height {
            return (state.ball.position.z - self.min_height).powf(self.exp)-1.
        } else {
            return 0.
        }
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}

/// trial at consecutive air-touch reward, hopefully should encourage air dribbling but may require more complexity
pub struct DribbleAirTouchReward {
    min_height: f64,
    exp: f64,
    max_touches: i64,
    max_rew_val: f64,
    curr_touch_vals: HashMap<i32, i64>
}

impl DribbleAirTouchReward {
    fn new(min_height_op: Option<f64>, exp_op: Option<f64>, max_touch_op: Option<i64>, max_rew_val_op: Option<f64>) -> Self {
        let min_height = match min_height_op {
            Some(min_height) => min_height,
            None => 93.
        };
        let exp = match exp_op {
            Some(exp) => exp,
            None => 0.2
        };
        let max_touch = match max_touch_op {
            Some(val) => val,
            None => 20
        };
        let max_rew_val = match max_rew_val_op {
            Some(val) => val,
            None => 1.0
        };
        let mut curr_touch_vals = HashMap::new();
        for i in 0..6 {
            curr_touch_vals.insert(i, 0);
        }

        DribbleAirTouchReward {
            min_height: min_height,
            exp: exp,
            max_touches: max_touch,
            max_rew_val: max_rew_val,
            curr_touch_vals: curr_touch_vals
        }
    }
}

impl RewardFn for DribbleAirTouchReward {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, _previous_action: &Vec<f64>) -> f64 {
        if player.ball_touched && !player.on_ground && state.ball.position.z >= self.min_height {
            let curr_touch_val = self.curr_touch_vals.get(&player.car_id).unwrap() + 1;
            if curr_touch_val > self.max_touches {
                return self.max_rew_val;
            } else {
                self.curr_touch_vals.insert(player.car_id, curr_touch_val);

                let mult = self.max_touches as f64 / curr_touch_val as f64;
                return (self.max_rew_val * mult).powf(self.exp);
            }
        } else {
            self.curr_touch_vals.insert(player.car_id, 0);
            return 0.
        }
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}

/// reward for gathering boost, based on boost amount instead of collecting pads
pub struct GatherBoostReward {
    last_boost: HashMap<i32, f64>
}

impl GatherBoostReward{
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        for i in 0..6 {
            hashmap.insert(i, 34.);
        }
        GatherBoostReward { last_boost: hashmap }
    }
}

impl RewardFn for GatherBoostReward {
    fn reset(&mut self,  _initial_state: &GameState) {
        for player in &_initial_state.players {
            // I know 34. is not correct (it should be 0.34) but it assures that the boost amount 
            // the agent is set with is above the starting amount so as to not reward resets
            self.last_boost.insert(player.car_id, 34.);
        }
    }

    fn get_reward(&mut self, player: &PlayerData, _state: &GameState, _previous_action: &Vec<f64>) -> f64 {
        let last_boost = self.last_boost.insert(player.car_id, player.boost_amount).unwrap();
        let boost_differential: f64;
        if player.boost_amount > last_boost {
            boost_differential = player.boost_amount - last_boost;
        } else {
            boost_differential = 0.;
        }
        return boost_differential
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}

/// "Wrapper" that collects a set of boxed reward functions and iterates through them to get a single float. 
/// Has other functionality including reward logging that sends info to a separate singular thread which writes for all instances
/// instead of each instance having its own writer
pub struct SB3CombinedLogRewardMultInst {
    // reward_file_path: PathBuf,
    // reward_file: String,
    // lockfile: String,
    reward_sender: Sender<Vec<f64>>,
    final_mult: f64,
    returns: Vec<f64>,
    combined_reward_fns: Vec<Box<dyn RewardFn + Send>>,
    combined_reward_weights: Vec<f64>
}

impl SB3CombinedLogRewardMultInst {
    fn new(reward_structs: Vec<Box<dyn RewardFn + Send>>, reward_weights: Vec<f64>, file_location: Option<String>, final_mult: Option<f64>, sender: Sender<Vec<f64>>) -> Self {
        // let file_location = match file_location {
        //     Some(file_location) => file_location,
        //     None => "./combinedlogfiles".to_owned()
        // };

        // let reward_file = format!("{}/rewards.txt", file_location);
        // let reward_file_path = Path::new(&reward_file);
        // let lockfile = format!("{}/reward_lock", file_location);
        
        let final_mult = match final_mult {
            Some(final_mult) => final_mult,
            None => 1.
        };
        // let exists = Path::new(&file_location).exists();
        // if !exists {
        //     let res = create_dir(&file_location);
        //     match res {
        //         Err(error) => {if error.kind() == AlreadyExists {} else {panic!("{error}")}}
        //         Ok(out) => out
        //     }
        // }
        // for i in 0..100 {
        //     if i == 99 {
        //         panic!("too many attempts taken to lock the file in new")
        //     }

        //     let out = OpenOptions::new().create(true).write(true).truncate(true).open(&reward_file_path);

        //     let file = match out {
        //         Err(out) => {if out.kind() == PermissionDenied {continue} else {println!("{out}");continue}},
        //         Ok(_file) => _file
        //     };

        //     let out = file.lock_exclusive();

        //     match out {
        //         Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
        //         Ok(_file) => _file
        //     };

        //     file.unlock().unwrap();
        //     break
        // }

        SB3CombinedLogRewardMultInst {
            // reward_file_path: reward_file_path.to_owned(),
            // reward_file: reward_file,
            // lockfile: lockfile,
            reward_sender: sender,
            final_mult: final_mult,
            returns: vec![0.; reward_structs.len()],
            combined_reward_fns: reward_structs,
            combined_reward_weights: reward_weights
        }
    }

    // fn file_put(returns_local: &Vec<f64>, reward_file: &String) {
    //     for i in 0..100 {
    //         if i == 99 {
    //             panic!("too many attempts taken to lock the file")
    //         }
    //         let out = File::open(&reward_file);

    //         let file = match out {
    //             Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
    //             Ok(_file) => _file
    //         };

    //         let out = file.lock_exclusive();

    //         match out {
    //             Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
    //             Ok(_file) => _file
    //         };

    //         let mut buf = BufWriter::new(&file);

    //         let mut string = String::new();
    //         string = string + "[";
    //         for i in 0..returns_local.len()-2 {
    //             string = string + &format!("{}, ", returns_local[i])
    //         }
    //         string = string + &format!("{}]", returns_local[returns_local.len()-1]);
    //         writeln!(&mut buf, "{}", string).unwrap();

    //         file.unlock().unwrap();
    //         break
    //     }
    // }
}

impl RewardFn for SB3CombinedLogRewardMultInst {
    fn reset(&mut self, _initial_state: &GameState) {
        // self.returns = vec![0.; self.combined_reward_fns.len()];
        for func in &mut self.combined_reward_fns {
            func.reset(_initial_state);
        }
        self.returns.fill(0.);
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        let mut final_val = 0.;
        // let final_val: f64;

        for (i, func) in self.combined_reward_fns.iter_mut().enumerate() {
            let val = func.get_reward(player, state, previous_action);
            let reward = val * self.combined_reward_weights[i];
            self.returns[i] += reward;
            final_val += reward;
        }
        // final_val = self.combined_reward_fns.par_iter_mut().zip(&mut self.returns).zip(&mut self.combined_reward_weights).map(|((func, ret), weight)| {
        //     let val = func.get_reward(player, state, previous_action);
        //     let reward = val * *weight;
        //     *ret += reward;
        //     reward
        // }).sum();
        
        // let vals = element_mult_vec(&rewards, &self.combined_reward_weights);
        // self.returns = element_add_vec(&self.returns, &vals);
        // let sum = vals.clone().iter().sum::<f64>();
        // let final_val = sum * self.final_mult; 

        return final_val * self.final_mult;
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        // let mut rewards = Vec::<f64>::new();
        let mut final_val = 0.;
        // let final_val: f64;

        // for func in &mut self.combined_reward_fns {
        //     let val = func.get_final_reward(player, state, previous_action);
        //     rewards.push(val);        
        // }

        for (i, func) in self.combined_reward_fns.iter_mut().enumerate() {
            let val = func.get_final_reward(player, state, previous_action);
            let reward = val * self.combined_reward_weights[i];
            self.returns[i] += reward;
            final_val += reward;
        }
        // final_val = self.combined_reward_fns.par_iter_mut().zip(&mut self.returns).zip(&mut self.combined_reward_weights).map(|((func, ret), weight)| {
        //     let val = func.get_final_reward(player, state, previous_action);
        //     let reward = val * *weight;
        //     *ret += reward;
        //     reward
        // }).sum();
        
        // let vals = element_mult_vec(&rewards, &self.combined_reward_weights);
        // self.returns = element_add_vec(&self.returns, &vals);
        // let local_ret = element_add_vec(&self.returns, &vals);
        let local_ret = self.returns.clone();
        
        // let local_ret = self.returns.clone();
        self.returns.fill(0.);
        // let reward_file = self.reward_file_path.clone();
        
        // thread::spawn(move || file_put(local_ret, reward_file.as_path()));
        self.reward_sender.send(local_ret).unwrap();

        // let sum = vals.clone().iter().sum::<f64>();
        // let final_val = sum * self.final_mult; 

        return final_val * self.final_mult;    
    }
}

// fn file_put(returns_local: Vec<f64>, reward_file: &Path) {
//     for i in 0..100 {
//         if i == 99 {
//             panic!("too many attempts taken to lock the file in file_put")
//         }
//         let out = OpenOptions::new().append(true).read(true).open(reward_file);

//         let file = match out {
//             Err(out) => {
//                 println!("file error: {out}");
//                 if out.kind() == PermissionDenied {continue} else {continue};},
//             Ok(_file) => _file
//         };

//         let out = file.lock_exclusive();

//         match out {
//             Err(out) => {
//                 println!("lock error: {out}");
//                 if out.kind() == PermissionDenied {continue} else {continue};},
//             Ok(_file) => _file
//         };

//         let mut buf = BufWriter::new(&file);

//         let mut string = String::new();
//         string = string + "[";
//         for i in 0..returns_local.len()-1 {
//             string = string + &format!("{}, ", returns_local[i])
//         }
//         string = string + &format!("{}]", returns_local[returns_local.len()-1]);
//         writeln!(&mut buf, "{}", string).unwrap();
//         let out = buf.flush();
//         match out {
//             Ok(out) => out,
//             Err(err) => println!("buf.flush in logger failed with error: {err}")
//         };
//         file.unlock().unwrap();
//         break
//     }
// }

/// "Wrapper" that collects a set of boxed reward functions and iterates through them to get a single float. 
/// Has other functionality including reward logging. Unlike the mult-instance version, it writes on its own new thread.
pub struct SB3CombinedLogReward {
    reward_file_path: PathBuf,
    // reward_file: String,
    // lockfile: String,
    final_mult: f64,
    returns: Vec<f64>,
    combined_reward_fns: Vec<Box<dyn RewardFn + Send>>,
    combined_reward_weights: Vec<f64>
}

impl SB3CombinedLogReward {
    fn new(reward_structs: Vec<Box<dyn RewardFn + Send>>, reward_weights: Vec<f64>, file_location: Option<String>, final_mult: Option<f64>) -> Self {
        let file_location = match file_location {
            Some(file_location) => file_location,
            None => "./combinedlogfiles".to_owned()
        };

        let reward_file = format!("{}/rewards.txt", file_location);
        let reward_file_path = Path::new(&reward_file);
        // let lockfile = format!("{}/reward_lock", file_location);
        
        let final_mult = match final_mult {
            Some(final_mult) => final_mult,
            None => 1.
        };
        let exists = Path::new(&file_location).exists();
        if !exists {
            let res = create_dir(&file_location);
            match res {
                Err(error) => {if error.kind() == AlreadyExists {} else {panic!("{error}")}}
                Ok(out) => out
            }
        }
        for i in 0..100 {
            if i == 99 {
                panic!("too many attempts taken to lock the file in new")
            }

            let out = OpenOptions::new().create(true).write(true).truncate(true).open(&reward_file_path);

            let file = match out {
                Err(out) => {if out.kind() == PermissionDenied {continue} else {println!("{out}");continue}},
                Ok(_file) => _file
            };

            let out = file.lock_exclusive();

            match out {
                Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
                Ok(_file) => _file
            };

            file.unlock().unwrap();
            break
        }

        SB3CombinedLogReward {
            reward_file_path: reward_file_path.to_owned(),
            // reward_file: reward_file,
            // lockfile: lockfile,
            final_mult: final_mult,
            returns: vec![0.; reward_structs.len()],
            combined_reward_fns: reward_structs,
            combined_reward_weights: reward_weights
        }
    }

    // fn file_put(returns_local: &Vec<f64>, reward_file: &String) {
    //     for i in 0..100 {
    //         if i == 99 {
    //             panic!("too many attempts taken to lock the file")
    //         }
    //         let out = File::open(&reward_file);

    //         let file = match out {
    //             Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
    //             Ok(_file) => _file
    //         };

    //         let out = file.lock_exclusive();

    //         match out {
    //             Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
    //             Ok(_file) => _file
    //         };

    //         let mut buf = BufWriter::new(&file);

    //         let mut string = String::new();
    //         string = string + "[";
    //         for i in 0..returns_local.len()-2 {
    //             string = string + &format!("{}, ", returns_local[i])
    //         }
    //         string = string + &format!("{}]", returns_local[returns_local.len()-1]);
    //         writeln!(&mut buf, "{}", string).unwrap();

    //         file.unlock().unwrap();
    //         break
    //     }
    // }
}

impl RewardFn for SB3CombinedLogReward {
    fn reset(&mut self, _initial_state: &GameState) {
        // self.returns = vec![0.; self.combined_reward_fns.len()];
        for func in &mut self.combined_reward_fns {
            func.reset(_initial_state);
        }
        self.returns.fill(0.);
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        let mut rewards = Vec::<f64>::new();

        for func in &mut self.combined_reward_fns {
            let val = func.get_reward(player, state, previous_action);
            rewards.push(val);
        }
        
        let vals = element_mult_vec(&rewards, &self.combined_reward_weights);
        self.returns = element_add_vec(&self.returns, &vals);
        let sum = vals.clone().iter().sum::<f64>();
        let final_val = sum * self.final_mult; 

        return final_val;
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        let mut rewards = Vec::<f64>::new();

        for func in &mut self.combined_reward_fns {
            let val = func.get_final_reward(player, state, previous_action);
            rewards.push(val);        
        }
        
        let vals = element_mult_vec(&rewards, &self.combined_reward_weights);
        // self.returns = element_add_vec(&self.returns, &vals);
        let local_ret = element_add_vec(&self.returns, &vals);
        
        // let local_ret = self.returns.clone();
        self.returns.fill(0.);
        let reward_file = self.reward_file_path.clone();
        
        thread::spawn(move || file_put(local_ret, reward_file.as_path()));

        let sum = vals.clone().iter().sum::<f64>();
        let final_val = sum * self.final_mult; 

        return final_val;    
    }
}

fn file_put(returns_local: Vec<f64>, reward_file: &Path) {
    for i in 0..100 {
        if i == 99 {
            panic!("too many attempts taken to lock the file in file_put")
        }
        let out = OpenOptions::new().append(true).read(true).open(reward_file);

        let file = match out {
            Err(out) => {
                println!("file error: {out}");
                if out.kind() == PermissionDenied {continue} else {continue};},
            Ok(_file) => _file
        };

        let out = file.lock_exclusive();

        match out {
            Err(out) => {
                println!("lock error: {out}");
                if out.kind() == PermissionDenied {continue} else {continue};},
            Ok(_file) => _file
        };

        let mut buf = BufWriter::new(&file);

        let mut string = String::new();
        string = string + "[";
        for i in 0..returns_local.len()-1 {
            string = string + &format!("{}, ", returns_local[i])
        }
        string = string + &format!("{}]", returns_local[returns_local.len()-1]);
        writeln!(&mut buf, "{}", string).unwrap();
        let out = buf.flush();
        match out {
            Ok(out) => out,
            Err(err) => println!("buf.flush in logger failed with error: {err}")
        };
        file.unlock().unwrap();
        break
    }
}
