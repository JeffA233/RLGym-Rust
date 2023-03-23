// use ndarray::*;
// use std::collections::VecDeque;
use std::f64::consts::PI;

use crate::common_values;
use crate::gamestates::game_state::GameState;
use crate::gamestates::physics_object::PhysicsObject;
use crate::gamestates::player_data::PlayerData;

use super::obs_builder::ObsBuilder;

/// Matrix's observation builder, holds a stack of previous ball positions and shows the stack in the observation
pub struct AdvancedObs {
    pos_std: f64,
    ang_std: f64,
}

impl AdvancedObs {
    // pub fn new(team_size: Option<usize>, expanding: Option<bool>, stack_size: Option<usize>) -> Self {
    pub fn new() -> Self {
        // let expanding = match expanding {
        //     Some(expanding) => expanding,
        //     None => false
        // };

        let advobsps = AdvancedObs {
            pos_std: 2300.,
            ang_std: PI,
        };
        return advobsps
    }

    fn _add_player_to_obs(&self, obs: &mut Vec<f64>, car: &PlayerData, ball: &PhysicsObject, inverted: bool, player: Option<&PhysicsObject>) -> PhysicsObject {
        let mut player_car: PhysicsObject;
        if inverted {
            player_car = car.inverted_car_data;
        } else {
            player_car = car.car_data;
        }

        let mut rel_pos = ball.position - player_car.position;
        rel_pos = rel_pos.divide_by_var(self.pos_std);
        let mut rel_vel = ball.linear_velocity - player_car.linear_velocity;
        rel_vel = rel_vel.divide_by_var(self.pos_std);
        
        obs.extend(rel_pos.into_array().iter());
        obs.extend(rel_vel.into_array().iter());
        obs.extend(player_car.position.divide_by_var(self.pos_std).into_array().iter());
        obs.extend(player_car.forward().iter());
        obs.extend(player_car.up().iter());
        obs.extend(player_car.linear_velocity.divide_by_var(self.pos_std).into_array().iter());
        obs.extend(player_car.angular_velocity.divide_by_var(self.ang_std).into_array().iter());
        obs.extend(vec![car.boost_amount, car.on_ground as i32 as f64, car.has_flip as i32 as f64, car.is_demoed as i32 as f64]);

        match player {
            Some(player) => {
                obs.extend((player_car.position - player.position).divide_by_var(self.pos_std).into_array().iter());
                obs.extend((player_car.linear_velocity - player.linear_velocity).divide_by_var(self.pos_std).into_array().iter());
            }
            None => ()
        };

        return player_car
    }
}

impl ObsBuilder for AdvancedObs {
    fn reset(&mut self, _initial_state: &GameState) {
        
    }

    fn get_obs_space(&mut self) -> Vec<usize> {
        vec![276]
    }

    fn build_obs(&mut self, player: &PlayerData, state: &GameState, config: &crate::envs::game_match::GameConfig, previous_action: &Vec<f64>) -> Vec<f64> {
        let inverted: bool;
        let ball: &PhysicsObject;
        let pads: [f64; 34];
        if player.team_num == common_values::ORANGE_TEAM {
           inverted = true;
           ball = &state.inverted_ball;
           pads = state.inverted_boost_pads; 
        } else {
            inverted = false;
            ball = &state.ball;
            pads = state.inverted_boost_pads;
        }

        let pos = &ball.position;
        let lin = &ball.linear_velocity;
        let ang = &ball.angular_velocity;

        // let pos_std = vec_div_variable(pos, &self.pos_std);
        // let lin_std = vec_div_variable(lin, &self.pos_std);
        // let ang_std = vec_div_variable(ang, &self.ang_std);
        let pos_std = pos.divide_by_var(self.pos_std);
        let lin_std = lin.divide_by_var(self.pos_std);
        let ang_std = ang.divide_by_var(self.ang_std);

        let mut obs = Vec::<f64>::with_capacity(276);

        obs.extend(pos_std.into_array().iter());
        obs.extend(lin_std.into_array().iter());
        obs.extend(ang_std.into_array().iter());
        obs.extend(previous_action.iter());
        obs.extend(pads.iter());

        // self.add_ball_to_stack(pos_std, lin_std, ang_std, player.car_id as usize);

        // let ball_stack = self.ball_stack[player.car_id as usize].make_contiguous().as_ref();
        // for ball_vec in self.ball_stack[player.car_id as usize].make_contiguous().as_ref() {

        let player_car = self._add_player_to_obs(&mut obs, &player, &ball, inverted, None);

        for other in &state.players {
            if other.car_id == player.car_id {
                continue;
            }

            self._add_player_to_obs(&mut obs, &other, ball, inverted, Some(&player_car));
        }

        return obs
    }
}