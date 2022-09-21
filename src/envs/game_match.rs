use std::{thread::spawn, any::Any};
use crate::{obs_builders::{aspo4_array::AdvancedObsPadderStacker, obs_builder::ObsBuilder}, action_parsers::necto_parser_2::NectoAction, conditionals::custom_conditions::CombinedTerminalConditions, reward_functions::default_reward::RewardFn};

use ndarray::*;

use crate::gamestates::{game_state::GameState, player_data::PlayerData};

pub struct GameMatch {
    pub _game_speed: usize,
    pub _gravity: f32,
    pub _boost_consumption: f32,
    pub _team_size: usize,
    pub _spawn_opponents: bool,
    pub _tick_skip: usize,
    pub _reward_fn: Box<dyn RewardFn>,
    pub _terminal_condition: CombinedTerminalConditions,
    pub _obs_builder: Box<dyn ObsBuilder>,
    pub _action_parser: NectoAction,
    pub _state_setter: fn() -> Vec<f32>,
    pub agents: usize,
    pub observation_space: Vec<usize>,
    pub action_space: Vec<usize>,
    pub _prev_actions: Array2<f32>,
    pub _spectator_ids: Vec<usize>,
    pub last_touch: usize,
    pub _initial_score: i64
}

// pub trait ObsMethods {
//     fn new() -> Self;
//     fn reset();
//     fn build_obs() -> dyn Any;
// }

// pub trait RewardFuncMethods {
//     fn new() -> Self;
//     fn reset(initial_state: GameState);
//     fn get_reward(&mut self, player: PlayerData, state: GameState, previous_action: Array1<f32>) -> f32;

// }

impl GameMatch {
    pub fn new(reward_function: Box<dyn RewardFn>, terminal_condition: CombinedTerminalConditions, obs_builder: Box<dyn ObsBuilder>, action_parser: NectoAction, state_setter: fn() -> Vec<f32>, team_size: Option<usize>, tick_skip: Option<usize>, game_speed: Option<usize>, gravity: Option<f32>, boost_consumption: Option<f32>, spawn_opponents: Option<bool>) -> Self {
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
            None => 100
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
            _prev_actions: Array2::<f32>::zeros((num_agents, 8)),
            _spectator_ids: Vec::<usize>::new(),
            last_touch: 100,
            _initial_score: 0,
        }
    }

    pub fn get_config(&self) -> Vec<f32> {
        let spawn_opponents_bool = if self._spawn_opponents {1} else {0};
        return vec![self._team_size as f32, 
        spawn_opponents_bool as f32, 
        self._tick_skip as f32,
        self._game_speed as f32,
        self._gravity as f32,
        self._boost_consumption as f32]
    }
}
