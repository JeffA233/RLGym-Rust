use crate::math::clip;
use crate::gamestates::game_state::GameState;
use super::action_parser::ActionParser;

#[derive(Clone, Copy, Default)]
pub struct ContinuousAction;

impl ContinuousAction {
    pub fn new() -> Self {
        ContinuousAction
    }
}

impl ActionParser for ContinuousAction {
    fn parse_actions(&mut self, actions: Vec<Vec<f64>>, _state: &GameState) -> Vec<Vec<f64>> {

        let mut parsed_actions = Vec::<Vec<f64>>::new();
        for action_vec in actions {
            parsed_actions.push(clip(action_vec, 1., -1.));
        }
        return parsed_actions
    }

    fn get_action_space(&mut self) -> Vec<f64> {
        return vec![]
    }
}
