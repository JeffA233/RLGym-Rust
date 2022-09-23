use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, common_values::BLUE_TEAM, math::{element_add_vec, element_mult_vec}};

use super::{common_rewards::{player_ball_rewards::VelocityPlayerToBallReward, ball_goal_rewards::VelocityBallToGoalReward, misc_rewards::{SaveBoostReward, VelocityReward, EventReward}}, default_reward::RewardFn};

// use numpy::*;
// use ndarray::*;
use std::{fs::*, path::PathBuf};
use std::io::{BufWriter, Write};
use std::io::ErrorKind::*;
use std::fs::File;
use fs2::FileExt;
use std::thread;
use std::path::Path;


pub fn get_custom_reward_func() -> Box<dyn RewardFn> {
    let mut reward_fn_vec = Vec::<Box<dyn RewardFn>>::new();

    reward_fn_vec.push(Box::new(VelocityPlayerToBallReward::new(None)));
    reward_fn_vec.push(Box::new(VelocityBallToGoalReward::new(None, None)));
    reward_fn_vec.push(Box::new(GatherBoostReward::new()));
    reward_fn_vec.push(Box::new(SaveBoostReward::new()));
    reward_fn_vec.push(Box::new(LeftKickoffReward::new()));
    reward_fn_vec.push(Box::new(JumpTouchReward::new(Some(100.), None)));
    reward_fn_vec.push(Box::new(VelocityReward::new(None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, None, None, None, Some(5.), Some(45.), Some(25.), None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, Some(100.), None, None, None, None, None, None)));
    reward_fn_vec.push(Box::new(EventReward::new(None, None, Some(-100.), None, None, None, None, None)));
    reward_fn_vec.push(Box::new(JumpReward::new()));
    // SB3CombinedLogReward {
    //     reward_file: "combinedlogfiles-v2".to_string(),
    //     final_mult: 0.1,
    //     returns: Vec::<f32>::new()
    // }

    Box::new(SB3CombinedLogReward::new(
        reward_fn_vec, 
        vec![0.05, 0.2, 5.0, 0.01, 1.0, 2.0, 0.02, 1.0, 1.0, 1.0, 0.006],
        Some("combinedlogfiles-v2".to_string()),
        Some(0.1)
    ))
}

pub struct JumpReward {}

impl JumpReward {
    pub fn new() -> Self {
        JumpReward {}
    }
}

impl RewardFn for JumpReward {
    fn reset(&mut self, _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, _state: &GameState, _previous_action: Vec<f32>) -> f32 {
        return player.has_jump as i32 as f32
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        self.get_reward(player, state, previous_action)
    }
}


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

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        if state.ball.position[0] == 0. && state.ball.position[1] == 0. {
            if self.kickoff_id_blue == -1 || self.kickoff_id_orange == -1 {
                let mut blue_car: PlayerData = state.players[0].clone();
                let mut orange_car: PlayerData = state.players[1].clone();
                self.kickoff_id_blue = -1;
                self.kickoff_id_orange = -1;

                for car in &state.players {
                    if car.team_num == BLUE_TEAM {
                        blue_car = car.clone();
                    } else {
                        orange_car = car.clone();
                    }
                }

                for car in &state.players {
                    if car.team_num == blue_car.team_num {
                        if car.car_data.position[1] >= blue_car.car_data.position[1] &&
                        car.car_data.position[0] > blue_car.car_data.position[0] {
                            blue_car = car.clone();
                        }
                    }
                    if car.team_num == orange_car.team_num {
                        if car.inverted_car_data.position[1] >= orange_car.inverted_car_data.position[1] &&
                        car.inverted_car_data.position[0] > orange_car.inverted_car_data.position[0] {
                            orange_car = car.clone();
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

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        self.get_reward(player, state, previous_action)
    }
}


pub struct JumpTouchReward {
    min_height: f32,
    exp: f32
}

impl JumpTouchReward {
    fn new(min_height: Option<f32>, exp: Option<f32>) -> Self {
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

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, _previous_action: Vec<f32>) -> f32 {
        if player.ball_touched && !player.on_ground && state.ball.position[2] >= self.min_height {
            return (state.ball.position[2] - 92.).powf(self.exp)-1.
        } else {
            return 0.
        }
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        self.get_reward(player, state, previous_action)
    }
}


pub struct GatherBoostReward {
    last_boost: f32
}

impl GatherBoostReward{
    pub fn new() -> Self {
        GatherBoostReward { last_boost: 34. }
    }
}

impl RewardFn for GatherBoostReward {
    fn reset(&mut self,  _initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, _state: &GameState, _previous_action: Vec<f32>) -> f32 {
        let boost_differential: f32;
        if player.boost_amount > self.last_boost {
            boost_differential = player.boost_amount - self.last_boost;
            self.last_boost = player.boost_amount;
        } else {
            boost_differential = 0.;
            self.last_boost = player.boost_amount;
        }
        return boost_differential/100.
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        self.get_reward(player, state, previous_action)
    }
}


pub struct SB3CombinedLogReward {
    reward_file_path: PathBuf,
    // reward_file: String,
    // lockfile: String,
    final_mult: f32,
    returns: Vec<f32>,
    combined_reward_fns: Vec<Box<dyn RewardFn>>,
    combined_reward_weights: Vec<f32>
}

impl SB3CombinedLogReward {
    fn new(reward_structs: Vec<Box<dyn RewardFn>>, reward_weights: Vec<f32>, file_location: Option<String>, final_mult: Option<f32>) -> Self {
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

            let out = OpenOptions::new().create(true).write(true).open(&reward_file_path);

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

    // fn file_put(returns_local: &Vec<f32>, reward_file: &String) {
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
        self.returns = vec![0.; self.combined_reward_fns.len()];
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        let mut rewards = Vec::<f32>::new();

        for func in &mut self.combined_reward_fns {
            rewards.push(func.get_reward(player, state, previous_action.clone()));
        }
        
        self.returns = element_add_vec(&self.returns, &rewards);
        self.returns = element_mult_vec(&self.returns, &self.combined_reward_weights);

        return self.returns.iter().sum::<f32>() * self.final_mult;
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        let mut rewards = Vec::<f32>::new();

        for func in &mut self.combined_reward_fns {
            rewards.push(func.get_reward(player, state, previous_action.clone()));
        }
        
        self.returns = element_add_vec(&self.returns, &rewards);
        self.returns = element_mult_vec(&self.returns, &self.combined_reward_weights);

        let local_ret = self.returns.clone();
        let reward_file = self.reward_file_path.clone();

        thread::spawn(move || file_put(&local_ret, reward_file.as_path()));

        return self.returns.iter().sum::<f32>() * self.final_mult;
    }
}

fn file_put(returns_local: &Vec<f32>, reward_file: &Path) {
    for i in 0..100 {
        if i == 99 {
            panic!("too many attempts taken to lock the file in file_put")
        }
        let out = File::open(reward_file);

        let file = match out {
            Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
            Ok(_file) => _file
        };

        let out = file.lock_exclusive();

        match out {
            Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
            Ok(_file) => _file
        };

        let mut buf = BufWriter::new(&file);

        let mut string = String::new();
        string = string + "[";
        for i in 0..returns_local.len()-2 {
            string = string + &format!("{}, ", returns_local[i])
        }
        string = string + &format!("{}]", returns_local[returns_local.len()-1]);
        writeln!(&mut buf, "{}", string).unwrap();

        file.unlock().unwrap();
        break
    }
}
