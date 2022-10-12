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
    fn reset(&mut self, initial_state: &GameState) {}

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        let vel = &player.car_data.linear_velocity;

        let pos_diff = state.ball.position.subtract(&player.car_data.position);

        if self.use_scalar_projection {
            return vel.scalar_projection(&pos_diff);
        } else {
            let partial = pos_diff.norm();
            let norm_pos_diff = pos_diff.divide_by_var(partial);
            let norm_vel = vel.divide_by_var(CAR_MAX_SPEED);
            return norm_pos_diff.multiply_by_vel(&norm_vel).into_array().iter().sum()
        }
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}