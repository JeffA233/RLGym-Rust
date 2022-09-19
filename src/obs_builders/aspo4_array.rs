use ndarray::*;
use std::collections::VecDeque;
use std::f32::consts::PI;

use crate::common_values;
use crate::gamestates::game_state::GameState;
use crate::gamestates::physics_object::PhysicsObject;
use crate::gamestates::player_data::PlayerData;

pub struct AdvancedObsPadderStacker {
    team_size: usize,
    pos_std: f32,
    ang_std: f32,
    expanding: bool,
    default_ball: Vec<Array1<f32>>,
    stack_size: usize,
    ball_stack: Vec<VecDeque<Vec<Array1<f32>>>>
}

impl AdvancedObsPadderStacker {
    pub fn new(team_size: Option<usize>, expanding: Option<bool>, stack_size: Option<usize>) -> Self {
        let team_size = match team_size {
            Some(team_size) => team_size,
            None => 3
        };
        let expanding = match expanding {
            Some(expanding) => expanding,
            None => false
        };
        let stack_size = match stack_size {
            Some(stack_size) => stack_size,
            None => 15
        };

        let advobsps = AdvancedObsPadderStacker {
            team_size: team_size,
            pos_std: 2300.,
            ang_std: PI,
            expanding: expanding,
            default_ball: vec![Array1::<f32>::zeros(3); 3],
            stack_size: stack_size,
            ball_stack: Vec::<VecDeque<Vec<Array1<f32>>>>::new()
        };
        for i in 0..66 {
            advobsps.blank_stack(i)
        }
        return advobsps
    }

    fn blank_stack(&mut self, index: usize) {
        for i in 0..self.stack_size {
            self.ball_stack[index].push_front(self.default_ball.clone())
        }
    }

    fn add_ball_to_stack(&mut self, pos_std: Array1<f32>, lin_std: Array1<f32>, ang_std: Array1<f32>, index: usize) {
        self.ball_stack[index].push_front(vec![pos_std, lin_std, ang_std]);
        self.ball_stack[index].truncate(self.stack_size);
    }

    fn reset() {
        
    }

    fn build_obs(&mut self, player: PlayerData, state: GameState, previous_action: Array1<f32>) {
        let inverted: bool;
        let ball: &PhysicsObject;
        let pads: Vec<f32>;
        if player.team_num == common_values::ORANGE_TEAM {
           inverted = true;
           ball = &state.inverted_ball;
           pads = state.inverted_boost_pads.clone(); 
        } else {
            inverted = false;
            ball = &state.ball;
            pads = state.inverted_boost_pads.clone();
        }

        let pos_std: Vec<f32> = ball.position.clone();
        let lin_std: Vec<f32> = ball.linear_velocity.clone();
        let ang_std: Vec<f32> = ball.angular_velocity.clone();

        let obs = Vec::<f32>::new();

        obs.append(&mut pos_std);
        obs.append(&mut lin_std);
        obs.append(&mut ang_std);
        obs.append(&mut previous_action.to_vec().clone());
        obs.append(&mut pads);

        let ball_stack = &self.ball_stack[player.car_id as usize].clone();
        
    }
}