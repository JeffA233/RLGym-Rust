use crate::{gamestates::{game_state::GameState, player_data::PlayerData}, envs::game_match::GameConfig};


pub trait ObsBuilder {
    fn reset(&mut self, initial_state: &GameState);
    fn get_obs_space(&mut self) -> Vec<usize>;
    fn pre_step(&mut self, state: &GameState, config: &GameConfig) {}
    fn build_obs(&mut self, player: &PlayerData, state: &GameState, config: &GameConfig, previous_action: &Vec<f64>) -> Vec<f64>;
}