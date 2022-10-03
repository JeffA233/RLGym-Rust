use crate::gamestates::game_state::GameState;


pub trait ActionParser {
    fn get_action_space(&mut self) -> Vec<f64>;
    fn parse_actions(&mut self, actions: Vec<Vec<f64>>, state: &GameState) -> Vec<Vec<f64>>;
}