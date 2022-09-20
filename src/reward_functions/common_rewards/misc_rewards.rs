use crate::{common_values::{BLUE_TEAM, CAR_MAX_SPEED}, gamestates::{player_data::PlayerData, game_state::GameState}, math::{element_sub_vec, element_mult_vec, norm_func}};
use std::collections::HashMap;


pub struct EventReward {
    weights: Vec<f32>,
    last_registered_values: HashMap<i32, Vec<f32>>
}

impl EventReward {
    pub fn new(goal: Option<f32>, team_goal: Option<f32>, concede: Option<f32>,
    touch: Option<f32>, shot: Option<f32>, save: Option<f32>, 
    demo: Option<f32>, boost_pickup: Option<f32>) -> Self {
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

    fn _extract_values(player: &PlayerData, state: &GameState) -> Vec<f32> {
        let team: i64;
        let opponent: i64;
        if player.team_num == BLUE_TEAM {
            team = state.blue_score;
            opponent = state.orange_score;
        } else {
            team = state.orange_score;
            opponent = state.blue_score;
        }

        return vec![player.match_goals as f32, team as f32, opponent as f32, player.ball_touched as i64 as f32, player.match_shots as f32,
        player.match_saves as f32, player.match_demolishes as f32, player.boost_amount]
    }

    pub fn reset(&mut self, _player: PlayerData, initial_state: GameState) {
        self.last_registered_values = HashMap::new();
        for player in &initial_state.players {
            self.last_registered_values.insert(player.car_id, EventReward::_extract_values(&player, &initial_state));
        }
    }

    pub fn get_reward(&mut self, player: PlayerData, state: GameState, _previous_action: Vec<f32>) -> f32 {
        let old_values  = self.last_registered_values.get(&player.car_id).unwrap();
        let new_values = EventReward::_extract_values(&player, &state);

        let diff_values = element_sub_vec(&new_values, old_values);

        self.last_registered_values.insert(player.car_id.clone(), new_values.clone());
        let is_value_positive: Vec<f32> = diff_values.iter().map(|x| if x > &0. {x.clone()} else {0.}).collect();
        let ret = element_mult_vec(&is_value_positive, &self.weights).iter().sum();
        return ret
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

    pub fn reset(&mut self, _initial_state: GameState) {}

    pub fn get_reward(&mut self, player: PlayerData, _state: GameState, _previous_action: Vec<f32>) -> f32 {
        return norm_func(&player.car_data.linear_velocity) / CAR_MAX_SPEED * (1 - 2 * self.negative as i32) as f32
    }
}

pub struct SaveBoostReward {}

impl SaveBoostReward {
    pub fn new() -> Self {
        SaveBoostReward {}
    }

    pub fn reset(&mut self, _initial_state: GameState) {}

    pub fn get_reward(&mut self, player: PlayerData, _state: GameState, _previous_action: Vec<f32>) -> f32 {
        return player.boost_amount.sqrt()
    }
}