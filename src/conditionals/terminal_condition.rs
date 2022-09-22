use crate::gamestates::game_state::GameState;



pub trait TerminalCondition {
    fn reset(&mut self, initial_state: &GameState);
    fn is_terminal(&mut self, current_state: &GameState) -> bool;
}