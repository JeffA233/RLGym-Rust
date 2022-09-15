use core::num;

use crate::gamestates::player_data::PlayerData;
use crate::gamestates::physics_object::PhysicsObject;

#[derive(Default)]
pub struct GameState {
    game_type: i64,
    blue_score: i64,
    orange_score: i64,
    last_touch: i32,
    players: Vec<PlayerData>,
    ball: PhysicsObject,
    inverted_ball: PhysicsObject,
    boost_pads: Vec<i64>,
    inverted_boost_pads: Vec<i64>
}

const BOOST_PAD_LENGTH: usize = 34;
const BALL_STATE_LENGTH: usize = 18;
const PLAYER_CAR_STATE_LENGTH: usize = 13;
const PLAYER_TERTIARY_INFO_LENGTH: usize = 11;
const PLAYER_INFO_LENGTH: usize = 2 + 2 * PLAYER_CAR_STATE_LENGTH + PLAYER_TERTIARY_INFO_LENGTH;

impl GameState {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn decode(mut self, state_vals: Vec<f64>) {
        let mut start = 3;
        let num_ball_packets = 1;
        let state_val_len = state_vals.len();

        let num_player_packets = (state_val_len - num_ball_packets * BALL_STATE_LENGTH - start - BOOST_PAD_LENGTH) / PLAYER_INFO_LENGTH;

        self.blue_score = state_vals[1] as i64;
        self.orange_score = state_vals[2] as i64;

        self.boost_pads = state_vals[start..start+BOOST_PAD_LENGTH].iter().map(|&x| x as i64).collect();
        self.inverted_boost_pads = self.boost_pads.clone();
        self.inverted_boost_pads.reverse();
        start = start + BOOST_PAD_LENGTH;

        self.ball.decode_ball_data(state_vals[start..start+BALL_STATE_LENGTH].to_vec());
        start = start + (BALL_STATE_LENGTH / 2);

        self.inverted_ball.decode_ball_data(state_vals[start..start+BALL_STATE_LENGTH].to_vec());
        start = start + (BALL_STATE_LENGTH / 2);

        for num in 0..num_player_packets {
            let player = self.decode_player(state_vals[start..start+PLAYER_INFO_LENGTH].to_vec());
            if player.ball_touched {
                self.last_touch = player.car_id as i32;
            }
            self.players.push(player);
            start = start + PLAYER_INFO_LENGTH;

        }

    }

    fn decode_player(&mut self, full_player_data: Vec<f64>) -> PlayerData {
        let mut player_data = PlayerData::default();

        let mut start: usize = 2;

        player_data.car_data.decode_car_data(full_player_data[start..start+PLAYER_CAR_STATE_LENGTH].to_vec());
        start = start + PLAYER_CAR_STATE_LENGTH;

        player_data.inverted_car_data.decode_car_data(full_player_data[start..start+PLAYER_CAR_STATE_LENGTH].to_vec());
        start = start + PLAYER_CAR_STATE_LENGTH;

        let tertiary_data = full_player_data[start..start+PLAYER_TERTIARY_INFO_LENGTH].to_vec();

        player_data.match_goals = tertiary_data[0] as i64;
        player_data.match_saves = tertiary_data[1] as i64;
        player_data.match_shots = tertiary_data[2] as i64;
        player_data.match_demolishes = tertiary_data[3] as i64;
        player_data.boost_pickups = tertiary_data[4] as i64;
        player_data.is_demoed = if tertiary_data[5] > 0. {
            true
        } else {
            false
        };
        player_data.on_ground = if tertiary_data[6] > 0. {
            true
        } else {
            false
        };
        player_data.ball_touched = if tertiary_data[7] > 0. {
            true
        } else {
            false
        };
        player_data.has_jump = if tertiary_data[8] > 0. {
            true
        } else {
            false
        };
        player_data.has_flip = if tertiary_data[9] > 0. {
            true
        } else {
            false
        };
        player_data.boost_amount = tertiary_data[10];
        player_data.car_id = full_player_data[0] as i32;
        player_data.team_num = full_player_data[1] as i32;
        
        return player_data
    }
}
