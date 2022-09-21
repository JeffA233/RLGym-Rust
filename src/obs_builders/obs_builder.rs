use crate::gamestates::{game_state::GameState, player_data::PlayerData};


pub trait ObsBuilder {
    fn reset(&mut self, initial_state: GameState);
    fn build_obs(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> Vec<f32>;
}