use ndarray::*;

pub struct game_match {
    pub _game_speed: usize,
    pub _gravity: f32,
    pub _boost_consumption: f32,
    pub _team_size: usize,
    pub _spawn_opponents: bool,
    pub _tick_skip: usize,
    pub _reward_fn: fn() -> f32,
    pub _terminal_conditions: fn() -> bool,
    pub _obs_builder: fn() -> Vec<f32>,
    pub _action_parser: fn() -> Vec<f32>,
    pub _state_setter: fn() -> Vec<f32>,
    pub agents: usize,
    pub observation_space: ArrayD<usize>,
    pub action_space: ArrayD<usize>,
    pub _prev_actions: ArrayD<usize>,
    pub _spectator_ids: Vec<usize>,
    pub last_touch: usize,
    pub _initial_score: i64
}

impl game_match {
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
