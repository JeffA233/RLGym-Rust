use crate::{common_values::{BLUE_TEAM, CAR_MAX_SPEED}, gamestates::{player_data::PlayerData, game_state::GameState}, math::{element_sub_vec, element_mult_vec, norm_func}, reward_functions::default_reward::RewardFn};
use std::collections::HashMap;


pub struct EventReward {
    weights: Vec<f64>,
    last_registered_values: HashMap<i32, Vec<f64>>
}

impl EventReward {
    pub fn new(goal: Option<f64>, team_goal: Option<f64>, concede: Option<f64>,
    touch: Option<f64>, shot: Option<f64>, save: Option<f64>, 
    demo: Option<f64>, boost_pickup: Option<f64>) -> Self {
        let goal = match goal {
            Some(goal) => goal,
            None => 0.
        };
        let team_goal = match team_goal {
            Some(team_goal) => team_goal,
            None => 0.
        };
        let concede = match concede {
            Some(concede) => concede,
            None => 0.
        };
        let touch = match touch {
            Some(touch) => touch,
            None => 0.
        };
        let shot = match shot {
            Some(shot) => shot,
            None => 0.
        };
        let save = match save {
            Some(save) => save,
            None => 0.
        };
        let demo = match demo {
            Some(demo) => demo,
            None => 0.
        };
        let boost_pickup = match boost_pickup {
            Some(boost_pickup) => boost_pickup,
            None => 0.
        };
        
        EventReward {
            weights: vec![goal, team_goal, concede, touch, shot, save, demo, boost_pickup],
            last_registered_values: HashMap::new()
        }
    }

    fn _extract_values(player: &PlayerData, state: &GameState) -> Vec<f64> {
        let team: i32;
        let opponent: i32;
        if player.team_num == BLUE_TEAM {
            team = state.blue_score;
            opponent = state.orange_score;
        } else {
            team = state.orange_score;
            opponent = state.blue_score;
        }

        return vec![player.match_goals as f64, team as f64, opponent as f64, player.ball_touched as i64 as f64, player.match_shots as f64,
        player.match_saves as f64, player.match_demolishes as f64, player.boost_amount]
    }
}

impl RewardFn for EventReward {
    fn reset(&mut self, initial_state: &GameState) {
        self.last_registered_values.clear();
        for player in &initial_state.players {
            let id = player.car_id;
            self.last_registered_values.insert(id, EventReward::_extract_values(&player, &initial_state));
        }
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        let id = player.car_id;
        // let is_empty = self.last_registered_values.is_empty();
        // let weights: Vec<String> = self.weights.iter().map(|x| x.to_string()).collect();
        // let weights_str = weights.join(" ");
        // println!("weights: {weights_str}");
        // self.weights[0] = 0.5;
        // println!("EventReward HashMap is_empty: {is_empty}");
        // let old_values = self.last_registered_values.get(&id);
        // let values;
        // let old_values = match old_values {
        //     Some(old_values) => old_values,
        //     None => {
        //         // this always goes to the None branch for some reason??
        //         println!("car_id value not found for EventReward, setting new value for car_id: {id}");
        //         // panic!("could not get values for event reward hashmap")
        //         values = EventReward::_extract_values(&player, &state);
        //         self.last_registered_values.insert(id, values);
        //         self.last_registered_values.get(&id).unwrap()
        //     }
        // };
        let new_values = EventReward::_extract_values(&player, &state);
        let old_values = self.last_registered_values.insert(id, new_values.clone());
        let old_values = match old_values {
            Some(old_values) => old_values,
            None => {
                // println!("car_id value not found for EventReward, setting new value for car_id: {id}");
                // panic!("could not get values for event reward hashmap")
                EventReward::_extract_values(&player, &state)
            }
        };
        let diff_values = element_sub_vec(&new_values, &old_values);

        self.last_registered_values.insert(id, new_values);
        let is_value_positive: Vec<f64> = diff_values.iter().map(|x| if *x > 0. {*x} else {0.}).collect();
        let ret = element_mult_vec(&is_value_positive, &self.weights).iter().sum();
        return ret 
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}


pub struct VelocityReward {
    negative: bool
}

impl VelocityReward {
    pub fn new(negative: Option<bool>) -> Self {
        let negative = match negative {
            Some(negative) => negative,
            None => false
        };
        VelocityReward { negative: negative }
    }
}

impl RewardFn for VelocityReward {
    fn reset(&mut self, initial_state: &GameState) {
        
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        // let norm = norm_func(&player.car_data.linear_velocity);
        let norm = player.car_data.linear_velocity.norm();
        let norm_std = norm / CAR_MAX_SPEED * (1 - 2 * self.negative as i32) as f64;
        return norm_std;
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}


pub struct SaveBoostReward {}

impl SaveBoostReward {
    pub fn new() -> Self {
        SaveBoostReward {}
    }
}

impl RewardFn for SaveBoostReward {
    fn reset(&mut self, initial_state: &GameState) {
        
    }

    fn get_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        return player.boost_amount.sqrt()
    }

    fn get_final_reward(&mut self, player: &PlayerData, state: &GameState, previous_action: &Vec<f64>) -> f64 {
        self.get_reward(player, state, previous_action)
    }
}