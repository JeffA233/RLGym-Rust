use crate::gamestates::{game_state::GameState, player_data::PlayerData};



pub trait RewardFn {
    fn reset(&mut self, initial_state: GameState);
    fn get_reward(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> f32;
    fn get_final_reward(&mut self, player: PlayerData, state: GameState, previous_action: Vec<f32>) -> f32;
}