use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, common_values::BLUE_TEAM};

use super::common_rewards::player_ball_rewards::VelocityPlayerToBallReward;

use numpy::*;
use ndarray::*;
use std::io::prelude::*;
use std::fs::File;



pub struct JumpReward {}

impl JumpReward {
    pub fn new() -> Self {
        JumpReward {}
    }

    pub fn reset(&mut self, _initial_state: GameState) {}

    pub fn get_reward(&mut self, player: PlayerData, _state: GameState, _previous_action: Vec<f32>) -> f32 {
        return player.has_jump as i32 as f32
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

    pub fn reset(&mut self, _initial_state: GameState) {
        self.vel_dir_reward.reset(_initial_state);
    }

    pub fn get_reward(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> f32 {
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

    pub fn reset(&mut self, _initial_state: GameState) {}

    pub fn get_reward(&mut self, player: PlayerData, state: GameState, _previous_action: Vec<f32>) -> f32 {
        if player.ball_touched && !player.on_ground && state.ball.position[2] >= self.min_height {
            return (state.ball.position[2] - 92.).powf(self.exp)-1.
        } else {
            return 0.
        }
    }
}

struct SB3CombinedLogReward {
    file_location: String,
    lockfile: String,
    final_mult: f32,
    returns: Array1<f32>
}

impl SB3CombinedLogReward {
    fn new(reward_fns: Vec<fn() -> f32>) {
        
    }
}