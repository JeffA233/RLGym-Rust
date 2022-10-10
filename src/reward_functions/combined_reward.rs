use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, math::element_mult_vec};

use super::{default_reward::RewardFn};



pub struct CombinedReward {
    reward_structs: Vec<Box<dyn RewardFn>>,
    reward_weights: Vec<f64>
}

impl CombinedReward {
    pub fn new(reward_structs: Vec<Box<dyn RewardFn>>, reward_weights: Vec<f64>) -> Self {
        assert!(reward_structs.len() == reward_weights.len(), "reward functions did not match reward weights");
        CombinedReward {
            reward_structs: reward_structs,
            reward_weights: reward_weights
        }
    }
}

impl RewardFn for CombinedReward {
    fn reset(&mut self, initial_state: &GameState) {
        for struc in &mut self.reward_structs {
            struc.reset(initial_state);
        }
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        let mut rewards = Vec::<f64>::new();
        for struc in &mut self.reward_structs {
            rewards.push(struc.get_reward(player, state, previous_action));
        }
        let ret = element_mult_vec(&rewards, &self.reward_weights);
        return ret.iter().sum()
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        let mut rewards = Vec::<f64>::new();
        for struc in &mut self.reward_structs {
            rewards.push(struc.get_reward(player, state, previous_action));
        }
        let ret = element_mult_vec(&rewards, &self.reward_weights);
        return ret.iter().sum()
    }
}