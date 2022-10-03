// use std::{thread::{spawn, current}, any::Any, iter::Enumerate};
use crate::{obs_builders::{obs_builder::ObsBuilder}, action_parsers::{action_parser::ActionParser}, conditionals::{terminal_condition::TerminalCondition}, reward_functions::default_reward::RewardFn, state_setters::state_setter::StateSetter};

// use ndarray::*;
// use numpy::*;

use crate::gamestates::{game_state::GameState};

pub struct GameMatch {
    pub _game_speed: f64,
    pub _gravity: f64,
    pub _boost_consumption: f64,
    pub _team_size: usize,
    pub _spawn_opponents: bool,
    pub _tick_skip: usize,
    pub _reward_fn: Box<dyn RewardFn>,
    pub _terminal_condition: Box<dyn TerminalCondition>,
    pub _obs_builder: Box<dyn ObsBuilder>,
    pub _action_parser: Box<dyn ActionParser>,
    pub _state_setter: Box<dyn StateSetter>,
    pub agents: usize,
    pub observation_space: Vec<usize>,
    pub action_space: Vec<usize>,
    pub _prev_actions: Vec<Vec<f64>>,
    pub _spectator_ids: Vec<i32>,
    pub last_touch: i32,
    pub _initial_score: i32
}

// pub trait ObsMethods {
//     fn new() -> Self;
//     fn reset();
//     fn build_obs() -> dyn Any;
// }

// pub trait RewardFuncMethods {
//     fn new() -> Self;
//     fn reset(initial_state: GameState);
//     fn get_reward(&mut self, player: PlayerData, state: GameState, previous_action: Array1<f64>) -> f64;
// }

impl GameMatch {
    pub fn new(
        reward_function: Box<dyn RewardFn>, 
        terminal_condition: Box<dyn TerminalCondition>, 
        obs_builder: Box<dyn ObsBuilder>, 
        action_parser: Box<dyn ActionParser>, 
        state_setter: Box<dyn StateSetter>, 
        team_size: Option<usize>, 
        tick_skip: Option<usize>, 
        game_speed: Option<f64>, 
        gravity: Option<f64>, 
        boost_consumption: Option<f64>, 
        spawn_opponents: Option<bool>
    ) -> Self {
        let team_size = match team_size {
            Some(team_size) => team_size,
            None => 1
        };
        let tick_skip = match tick_skip {
            Some(tick_skip) => tick_skip,
            None => 8
        };
        let game_speed = match game_speed {
            Some(game_speed) => game_speed,
            None => 100.
        };
        let gravity = match gravity {
            Some(gravity) => gravity,
            None => 1.
        };
        let boost_consumption = match boost_consumption {
            Some(boost_consumption) => boost_consumption,
            None => 1.
        };
        let spawn_opponents = match spawn_opponents {
            Some(spawn_opponents) => spawn_opponents,
            None => true
        };
        let num_agents = if spawn_opponents {team_size * 2} else {team_size};
        
        GameMatch {
            _game_speed: game_speed,
            _gravity: gravity,
            _boost_consumption: boost_consumption,
            _team_size: team_size,
            _spawn_opponents: spawn_opponents,
            _tick_skip: tick_skip,
            _reward_fn: reward_function,
            _terminal_condition: terminal_condition,
            _obs_builder: obs_builder,
            _action_parser: action_parser,
            _state_setter: state_setter,
            agents: num_agents,
            observation_space: Vec::<usize>::new(),
            action_space: Vec::<usize>::new(),
            // _prev_actions: Array2::<f64>::zeros((num_agents, 8)),
            _prev_actions: vec![vec![0.; 8]; num_agents],
            _spectator_ids: vec![0; 6],
            last_touch: -1,
            _initial_score: 0,
        }
    }

    pub fn episode_reset(&mut self, initial_state: &GameState) {
        self._spectator_ids = initial_state.players.iter().map(|x| x.car_id).collect();
        self._prev_actions.fill(vec![0.; 8]);
        self._terminal_condition.reset(&initial_state);
        self._reward_fn.reset(&initial_state);
        self._obs_builder.reset(&initial_state);
        self.last_touch = -1;
        self._initial_score = initial_state.blue_score - initial_state.orange_score;
    }

    pub fn build_observations(&mut self, mut state: &mut GameState) -> Vec<Vec<f64>> {
        let mut observations = Vec::<Vec<f64>>::new();

        if state.last_touch == -1 {
            state.last_touch = self.last_touch.clone();
        } else {
            self.last_touch = state.last_touch.clone();
        }

        self._obs_builder.pre_step(&state);

        for (i, player) in state.players.iter().enumerate() {
            observations.push(self._obs_builder.build_obs(player, &state, self._prev_actions[i].clone()));
        }

        // if observations.len() == 1 {
        //     return observations
        // } else {
        //     return observations
        // }
        return observations
    }

    pub fn get_rewards(&mut self, state: &GameState, done: bool) -> Vec<f64> {
        let mut rewards = Vec::<f64>::new();

        self._reward_fn.pre_step(&state);

        for (i, player) in state.players.iter().enumerate() {
            if done {
                rewards.push(self._reward_fn.get_final_reward(player, &state, self._prev_actions[i].clone()));
            } else {
                rewards.push(self._reward_fn.get_reward(player, &state, self._prev_actions[i].clone()));
            }
        }

        // if rewards.len() == 1 {
        //     return vec![rewards[0]]
        // } else {
        //     return rewards
        // }
        return rewards
    }

    pub fn is_done(&mut self, state: &GameState) -> bool {
        if self._terminal_condition.is_terminal(&state) {
            return true
        } else {
            return false
        }
    }

    pub fn get_result(&self, state: GameState) -> i32 {
        let current_score = state.blue_score - state.orange_score;
        return current_score - self._initial_score;
    }

    pub fn parse_state(&self, state_floats: Vec<f64>) -> GameState {
        return GameState::new(Some(state_floats))
    }

    pub fn parse_actions(&mut self, actions: Vec<Vec<f64>>, state: &GameState) -> Vec<Vec<f64>> {
        return self._action_parser.parse_actions(actions, &state)
    }

    pub fn format_actions(&mut self, mut actions: Vec<Vec<f64>>) -> Vec<f64> {
        let mut acts = Vec::<f64>::new();

        self._prev_actions = actions.clone();

        for i in 0..actions.len() {
            acts.push(self._spectator_ids[i] as f64);
            acts.append(&mut actions[i]);
        }
        return acts
    }

    pub fn get_reset_state(&mut self) -> Vec<f64> {
        let mut new_state = self._state_setter.build_wrapper(self._team_size.clone(), self._spawn_opponents.clone());
        self._state_setter.reset(&mut new_state);
        return new_state.format_state()
    }

    pub fn get_config(&self) -> Vec<f64> {
        let spawn_opponents_bool = if self._spawn_opponents {1} else {0};
        return vec![self._team_size as f64, 
        spawn_opponents_bool as f64, 
        self._tick_skip as f64,
        self._game_speed as f64,
        self._gravity as f64,
        self._boost_consumption as f64]
    }

    pub fn update_settings(&mut self, game_speed: Option<f64>, gravity: Option<f64>, boost_consumption: Option<f64>) {
        match game_speed {
            Some(game_speed) => self._game_speed = game_speed,
            None => ()
        };
        match gravity {
            Some(gravity) => self._gravity = gravity,
            None => ()
        };
        match boost_consumption {
            Some(boost_consumption) => self._boost_consumption = boost_consumption,
            None => ()
        };
    }

    fn _auto_detech_obs_space(&mut self) {
        self.observation_space = self._obs_builder.get_obs_space();
    }
}
