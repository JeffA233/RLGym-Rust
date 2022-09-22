use crate::gamestates::game_state::GameState;

use super::action_parser::ActionParser;

pub struct NectoAction {
    _lookup_table: Vec<Vec<f32>>
}

impl NectoAction {
    pub fn new() -> Self {
        NectoAction {
            _lookup_table: NectoAction::make_lookup_table()
        }
    }

    fn make_lookup_table() -> Vec<Vec<f32>> {
        let mut actions = Vec::<Vec<f32>>::new();
        for throttle in [-1., 0., 1.] {
            for steer in [-1., 0., 1. as f32] {
                for boost in [0., 1. as f32] {
                    for handbrake in [0., 1. as f32] {
                        if boost == 1. && throttle != 1. {
                            continue
                        }
                        actions.push(vec![if throttle != 0. || boost!= 0. {throttle} else {0.},
                            steer, 0., steer, 0., 0., boost, handbrake]);
                    }
                }
            }
        }
        for pitch in [-1., 0., 1. as f32] {
            for yaw in [-1., 0., 1. as f32] {
                for roll in [-1., 0., 1. as f32] {
                    for jump in [0., 1. as f32] {
                        for boost in [0., 1. as f32] {
                            if jump == 1. && yaw != 0. {
                                continue
                            }
                            if pitch == roll && roll == jump && jump == 0. {
                                continue
                            }
                            let handbrake = if jump == 1. && (pitch != 0. || yaw != 0. || roll != 0.) {1.} else {0.};
                            actions.push(vec![boost, yaw, pitch, roll, jump, boost, handbrake]);
                        }
                    }
                }
            }
        }
        return actions
    }
}

impl ActionParser for NectoAction {
    fn get_action_space(&mut self) -> Vec<f32> {
        let mut action_space = Vec::<f32>::new();
        action_space.push(self._lookup_table.len() as f32);
        return action_space
    }

    fn parse_actions(&mut self, actions: Vec<Vec<f32>>, _state: &GameState) -> Vec<Vec<f32>> {
        let mut parsed_actions = Vec::<Vec<f32>>::new();
        for action_vec in actions {
            for action in action_vec {
                parsed_actions.push(self._lookup_table[action as usize].clone());
            }
        }
        return parsed_actions
    }
}