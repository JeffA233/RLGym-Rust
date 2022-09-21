use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, common_values::BLUE_TEAM};

use super::{common_rewards::player_ball_rewards::VelocityPlayerToBallReward, default_reward::RewardFn, combined_reward::CombinedReward};

use numpy::*;
use ndarray::*;
use std::fs::*;
use std::io::{BufWriter, Write};
use std::io::ErrorKind::*;
use std::fs::File;


pub struct JumpReward {}

impl JumpReward {
    pub fn new() -> Self {
        JumpReward {}
    }
}

impl RewardFn for JumpReward {
    fn reset(&mut self, _initial_state: GameState) {}

    fn get_reward(&mut self, player: PlayerData, _state: GameState, _previous_action: Vec<f32>) -> f32 {
        return player.has_jump as i32 as f32
    }

    fn get_final_reward(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> f32 {
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
    fn reset(&mut self, _initial_state: GameState) {
        self.vel_dir_reward.reset(_initial_state);
    }

    fn get_reward(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> f32 {
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

    fn get_final_reward(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> f32 {
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
    fn reset(&mut self, _initial_state: GameState) {}

    fn get_reward(&mut self, player: PlayerData, state: GameState, _previous_action: Vec<f32>) -> f32 {
        if player.ball_touched && !player.on_ground && state.ball.position[2] >= self.min_height {
            return (state.ball.position[2] - 92.).powf(self.exp)-1.
        } else {
            return 0.
        }
    }

    fn get_final_reward(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> f32 {
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
    fn reset(&mut self,  _initial_state: GameState) {}

    fn get_reward(&mut self, player: PlayerData, _state: GameState, _previous_action: Vec<f32>) -> f32 {
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

    fn get_final_reward(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> f32 {
        self.get_reward(player, state, previous_action)
    }
}

struct SB3CombinedLogReward {
    reward_file: String,
    // lockfile: String,
    final_mult: f32,
    returns: Array1<f32>,
    combined_reward_struct: CombinedReward
}

impl SB3CombinedLogReward {
    fn new(reward_structs: Vec<Box<dyn RewardFn>>, reward_weights: Vec<f32>, file_location: Option<String>, final_mult: Option<f32>) -> Self {
        let file_location = match file_location {
            Some(file_location) => file_location,
            None => "combinedlogfiles".to_owned()
        };

        let reward_file = format!("{}/rewards.txt", file_location);
        // let lockfile = format!("{}/reward_lock", file_location);
        
        let final_mult = match final_mult {
            Some(final_mult) => final_mult,
            None => 1.
        };
        while true {
            let out = OpenOptions::new().create(true).open(&reward_file);

            match out {
                Err(out) => {if out.kind() == PermissionDenied {continue} else {continue}},
                Ok(file) => break
            };
        }
        // let out = File::create(&logfile).unwrap();
        // let mut ret = BufWriter::new(out);
        // let vec = vec![1., 2.5, 3.5, 4.5, 6.6];
        // let mut string = String::new();
        // string = string + "[";
        // for i in 0..vec.len()-2 {
        //     string = string + &format!("{}, ", vec[i])
        // }
        // string = string + &format!("{}]", vec[vec.len()-1]);
        // writeln!(&mut ret, "{}", string).unwrap();

        SB3CombinedLogReward {
            reward_file: reward_file,
            // lockfile: lockfile,
            final_mult: final_mult,
            returns: Array1::<f32>::zeros(reward_structs.len()),
            combined_reward_struct: CombinedReward::new(reward_structs, reward_weights)
        }
    }
}

// impl RewardFn for SB3CombinedLogReward {

// }