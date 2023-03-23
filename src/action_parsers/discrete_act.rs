use super::action_parser::ActionParser;



pub struct DiscreteAction {
    n_bins: usize
}

impl DiscreteAction {
    pub fn new() -> Self {
        DiscreteAction { n_bins: 3 }
    }
}

impl ActionParser for DiscreteAction {
    fn get_action_space(&mut self) -> Vec<f64> {
        let mut act_space = vec![self.n_bins as f64; 5];
        act_space.extend([2.; 3]);
        act_space
    }

    fn parse_actions(&mut self, actions: Vec<Vec<f64>>, state: &crate::gamestates::game_state::GameState) -> Vec<Vec<f64>> {
        let mut parsed_actions = Vec::<Vec<f64>>::new();
        // [[self.n_bins; 5], bool, bool, bool]
        for mut action_vec in actions {
            // let act = &mut action_vec[0];
            for i in 0..5 {
                let num = &mut action_vec[i];
                *num = *num / (self.n_bins / 2) as f64 - 1.;
            }
            parsed_actions.push(action_vec);
        }

        return parsed_actions
    }
}