use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, math::{element_sub_vec, scalar_projection, norm_func, vec_div_variable, element_mult_vec}, common_values::CAR_MAX_SPEED, reward_functions::default_reward::RewardFn};



pub struct VelocityPlayerToBallReward {
    use_scalar_projection: bool
}

impl VelocityPlayerToBallReward {
    pub fn new(use_scalar_projection: Option<bool>) -> Self {
        let use_scalar_projection = match use_scalar_projection {
            Some(use_scalar_projection) => use_scalar_projection,
            None => false
        };
        VelocityPlayerToBallReward {
            use_scalar_projection: use_scalar_projection
        }
    }
}

impl RewardFn for VelocityPlayerToBallReward {
    fn reset(&mut self, initial_state: &GameState) {
        
    }
    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        let vel = &player.car_data.linear_velocity;

        let pos_diff = element_sub_vec(&state.ball.position, &player.car_data.position);

        if self.use_scalar_projection {
            return scalar_projection(&vel, &pos_diff)
        } else {
            let partial = norm_func(&pos_diff);
            let norm_pos_diff = vec_div_variable(&pos_diff, &partial);
            let norm_vel = vec_div_variable(&norm_pos_diff, &CAR_MAX_SPEED);
            return element_mult_vec(&norm_pos_diff, &norm_vel).iter().sum()
        }
    }
    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> f32 {
        self.get_reward(player, state, previous_action)
    }
}