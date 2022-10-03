use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, common_values::{BLUE_TEAM, ORANGE_TEAM, ORANGE_GOAL_BACK, BLUE_GOAL_BACK, BALL_MAX_SPEED}, math::element_sub_vec, reward_functions::default_reward::RewardFn};
use crate::math::*;


pub struct VelocityBallToGoalReward {
    own_goal: bool,
    use_scalar_projection: bool
}

impl VelocityBallToGoalReward {
    pub fn new(own_goal: Option<bool>, use_scalar_projection: Option<bool>) -> Self {
        let own_goal = match own_goal {
            Some(own_goal) => own_goal,
            None => false
        };
        let use_scalar_projection = match use_scalar_projection {
            Some(use_some_projection) => use_some_projection,
            None => false
        };
        VelocityBallToGoalReward {
            own_goal: own_goal,
            use_scalar_projection: use_scalar_projection
        }
    }
}

impl RewardFn for VelocityBallToGoalReward {
    fn reset(&mut self, initial_state: &GameState) {
        
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f64>) -> f64 {
        let objective: Vec<f64>;
        if (player.team_num == BLUE_TEAM && !self.own_goal) || (player.team_num == ORANGE_TEAM && self.own_goal) {
            objective = ORANGE_GOAL_BACK.to_vec();
        } else {
            objective = BLUE_GOAL_BACK.to_vec();
        }

        let pos_diff = element_sub_vec(&objective, &state.ball.position);

        if self.use_scalar_projection {
            return scalar_projection(&state.ball.linear_velocity, &pos_diff)
        } else {
            let pos_diff_normed = norm_func(&pos_diff);
            let norm_pos_diff = vec_div_variable(&pos_diff, &pos_diff_normed);
            let norm_vel = vec_div_variable(&state.ball.linear_velocity, &BALL_MAX_SPEED);
            return element_mult_vec(&norm_pos_diff, &norm_vel).iter().sum()
        }
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}