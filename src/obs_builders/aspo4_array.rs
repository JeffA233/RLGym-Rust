// use ndarray::*;
use std::collections::VecDeque;
use std::f64::consts::PI;

use crate::common_values;
use crate::gamestates::game_state::GameState;
use crate::gamestates::physics_object::PhysicsObject;
use crate::gamestates::player_data::PlayerData;
use crate::math::*;

use super::obs_builder::ObsBuilder;

pub struct AdvancedObsPadderStacker {
    team_size: usize,
    pos_std: f64,
    ang_std: f64,
    // expanding: bool,
    default_ball: Vec<Vec<f64>>,
    stack_size: usize,
    ball_stack: Vec<VecDeque<Vec<Vec<f64>>>>
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
            ball_stack: Vec::<VecDeque<Vec<Vec<f64>>>>::new()
        };
        for _i in 0..8 {
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

    fn add_ball_to_stack(&mut self, mut pos_std: Vec<f64>, mut lin_std: Vec<f64>, mut ang_std: Vec<f64>, index: usize) {
        // to match Python functionality unfortunately (using extendleft from deque)
        pos_std.reverse();
        lin_std.reverse();
        ang_std.reverse();

        self.ball_stack[index].push_front(vec![pos_std, lin_std, ang_std]);
        self.ball_stack[index].truncate(self.stack_size);
    }

    fn _add_dummy(obs: &mut Vec<f64>) {
        obs.extend([0.; 31]);
    }

    fn _add_player_to_obs(&self, obs: &mut Vec<f64>, car: &PlayerData, ball: &PhysicsObject, inverted: bool, player: Option<&PhysicsObject>) -> PhysicsObject {
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
        obs.append(&mut vec![car.boost_amount, car.on_ground as i32 as f64, car.has_flip as i32 as f64, car.is_demoed as i32 as f64]);

        match player {
            Some(player) => {
                obs.append(&mut vec_div_variable(&element_sub_vec(&player_car.position, &player.position), &self.pos_std));
                obs.append(&mut vec_div_variable(&element_sub_vec(&player_car.linear_velocity, &player.linear_velocity), &self.pos_std));
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

    fn build_obs(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> Vec<f64> {
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

        let pos_std = vec_div_variable(pos, &self.pos_std);
        let lin_std = vec_div_variable(lin, &self.pos_std);
        let ang_std = vec_div_variable(ang, &self.ang_std);

        let mut obs = Vec::<f64>::new();

        obs.extend(pos_std.iter());
        obs.extend(lin_std.iter());
        obs.extend(ang_std.iter());
        obs.extend(previous_action.iter());
        obs.extend(pads.iter());

        // self.add_ball_to_stack(pos_std, lin_std, ang_std, player.car_id as usize);

        let ball_stack = &self.ball_stack[player.car_id as usize];
        for ball_vec in ball_stack {
            let pos_std = &ball_vec[0];
            let lin_std = &ball_vec[1];
            let ang_std = &ball_vec[2];
            obs.extend(ang_std.iter());
            obs.extend(lin_std.iter());
            obs.extend(pos_std.iter());
        }

        self.add_ball_to_stack(pos_std, lin_std, ang_std, player.car_id as usize);

        let player_car = self._add_player_to_obs(&mut obs, &player, &ball, inverted, None);

        let mut ally_count = 0;
        let mut enemy_count = 0;

        for other in &state.players {
            if other.car_id == player.car_id {
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

            self._add_player_to_obs(&mut obs, &other, ball, inverted, Some(&player_car));
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