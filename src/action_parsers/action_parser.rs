use crate::gamestates::game_state::GameState;


pub trait ActionParser {
    fn get_action_space(&mut self) -> Vec<f32>;
    fn parse_actions(&mut self, actions: Vec<Vec<f32>>, state: &GameState) -> Vec<Vec<f32>>;
}