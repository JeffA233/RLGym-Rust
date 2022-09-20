use crate::gamestates::{game_state::GameState, player_data::PlayerData};

use super::common_rewards::player_ball_rewards::VelocityPlayerToBallReward;



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

    pub fn get_reward(&mut self, player: PlayerData, state: GameState, _previous_action: Vec<f32>) -> f32 {
        if state.ball.position[0] == 0. && state.ball.position[1] == 0. {
            if self.kickoff_id_blue == -1 || self.kickoff_id_orange == -1 {
                
            }
        }
    }
}