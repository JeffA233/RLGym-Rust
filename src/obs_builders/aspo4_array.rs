// use ndarray::*;
use std::collections::VecDeque;
use std::f32::consts::PI;

use crate::common_values;
use crate::gamestates::game_state::GameState;
use crate::gamestates::physics_object::PhysicsObject;
use crate::gamestates::player_data::PlayerData;
use crate::math::*;

use super::obs_builder::ObsBuilder;

pub struct AdvancedObsPadderStacker {
    team_size: usize,
    pos_std: f32,
    ang_std: f32,
    // expanding: bool,
    default_ball: Vec<Vec<f32>>,
    stack_size: usize,
    ball_stack: Vec<VecDeque<Vec<Vec<f32>>>>
}

impl AdvancedObsPadderStacker {
    // pub fn new(team_size: Option<usize>, expanding: Option<bool>, stack_size: Option<usize>) -> Self {
    pub fn new(team_size: Option<usize>, stack_size: Option<usize>) -> Self {
        let team_size = match team_size {
            Some(team_size) => team_size,
            None => 3
        };
        // let expanding = match expanding {
        //     Some(expanding) => expanding,
        //     None => false
        // };
        let stack_size = match stack_size {
            Some(stack_size) => stack_size,
            None => 15
        };

        let mut advobsps = AdvancedObsPadderStacker {
            team_size: team_size,
            pos_std: 2300.,
            ang_std: PI,
            // expanding: expanding,
            default_ball: vec![vec![0.; 3]; 3],
            stack_size: stack_size,
            ball_stack: Vec::<VecDeque<Vec<Vec<f32>>>>::new()
        };
        for i in 0..66 {
            advobsps.blank_stack()
        }
        return advobsps
    }

    fn blank_stack(&mut self) {
        let mut default_deque = VecDeque::new();
        for _i in 0..self.stack_size {
            default_deque.push_front(self.default_ball.clone());
        }
        self.ball_stack.push(default_deque)
        // for _ in 0..self.stack_size {
        //     self.ball_stack[index].push_front(self.default_ball.clone())
        // }
    }

    fn add_ball_to_stack(&mut self, pos_std: Vec<f32>, lin_std: Vec<f32>, ang_std: Vec<f32>, index: usize) {
        self.ball_stack[index].push_front(vec![pos_std, lin_std, ang_std]);
        self.ball_stack[index].truncate(self.stack_size);
    }

    fn _add_dummy(obs: &mut Vec<f32>) {
        obs.append(&mut vec![0.; 31]);
    }

    fn _add_player_to_obs(&self, obs: &mut Vec<f32>, car: &PlayerData, ball: &PhysicsObject, inverted: bool, player: Option<PhysicsObject>) -> PhysicsObject {
        let mut player_car: PhysicsObject;
        if inverted {
            player_car = car.inverted_car_data.clone();
        } else {
            player_car = car.car_data.clone();
        }

        let mut rel_pos = element_sub_vec(&ball.position, &player_car.position);
        rel_pos = vec_div_variable(&rel_pos, &self.pos_std);
        let mut rel_vel = element_sub_vec(&ball.linear_velocity, &player_car.linear_velocity);
        rel_vel = vec_div_variable(&rel_vel, &self.pos_std);

        obs.append(&mut rel_pos);
        obs.append(&mut rel_vel);
        obs.append(&mut vec_div_variable(&player_car.position, &self.pos_std));
        obs.append(&mut player_car.forward());
        obs.append(&mut player_car.up());
        obs.append(&mut vec_div_variable(&player_car.linear_velocity, &self.pos_std));
        obs.append(&mut vec_div_variable(&player_car.angular_velocity, &self.ang_std));
        obs.append(&mut vec![car.boost_amount, car.on_ground as i32 as f32, car.has_flip as i32 as f32, car.is_demoed as i32 as f32]);

        match player {
            Some(player) => {
                obs.append(&mut vec_div_variable(&element_div_vec(&player_car.position, &player.position), &self.pos_std));
                obs.append(&mut vec_div_variable(&element_div_vec(&player_car.linear_velocity, &player.linear_velocity), &self.pos_std));
            }
            None => ()
        };

        return player_car
    }
}

impl ObsBuilder for AdvancedObsPadderStacker {
    fn reset(&mut self, _initial_state: &GameState) {
        
    }

    fn get_obs_space(&mut self) -> Vec<usize> {
        vec![276]
    }

    fn build_obs(&mut self, player: &PlayerData, state: &GameState, previous_action: Vec<f32>) -> Vec<f32> {
        let inverted: bool;
        let ball: &PhysicsObject;
        let mut pads: Vec<f32>;
        if player.team_num == common_values::ORANGE_TEAM {
           inverted = true;
           ball = &state.inverted_ball;
           pads = state.inverted_boost_pads.clone(); 
        } else {
            inverted = false;
            ball = &state.ball;
            pads = state.inverted_boost_pads.clone();
        }

        // let mut pos_std: Vec<f32> = ball.position.clone();
        // let mut lin_std: Vec<f32> = ball.linear_velocity.clone();
        // let mut ang_std: Vec<f32> = ball.angular_velocity.clone();

        let mut obs = Vec::<f32>::new();

        obs.append(&mut ball.position.clone());
        obs.append(&mut ball.linear_velocity.clone());
        obs.append(&mut ball.angular_velocity.clone());
        obs.append(&mut previous_action.clone());
        obs.append(&mut pads);

        let ball_stack = self.ball_stack[player.car_id as usize].clone();
        for ball_vec in ball_stack {
            let mut pos_std = ball_vec[0].clone();
            let mut lin_std = ball_vec[1].clone();
            let mut ang_std = ball_vec[2].clone();
            obs.append(&mut pos_std);
            obs.append(&mut lin_std);
            obs.append(&mut ang_std);

        }
        self.add_ball_to_stack(ball.position.clone(), ball.linear_velocity.clone(), ball.angular_velocity.clone(), player.car_id as usize);

        let player_car = self._add_player_to_obs(&mut obs, &player, &ball, inverted, None);

        let mut ally_count = 0;
        let mut enemy_count = 0;

        for other in &state.players {
            if other.car_id == player.team_num {
                continue;
            }
            
            if other.team_num == player.team_num {
                ally_count += 1;
                if ally_count > self.team_size - 1 {
                    continue;
                }
            } else {
                enemy_count += 1;
                if enemy_count > self.team_size {
                    continue;
                }
            }

            self._add_player_to_obs(&mut obs, &other, ball, inverted, Some(player_car.clone()));
        }

        while ally_count < self.team_size - 1 {
            AdvancedObsPadderStacker::_add_dummy(&mut obs);
            ally_count += 1;
        }
        while enemy_count < self.team_size {
            AdvancedObsPadderStacker::_add_dummy(&mut obs);
            enemy_count += 1;
        }

        return obs
    }
}