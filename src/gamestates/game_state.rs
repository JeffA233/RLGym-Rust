// use core::num;

use crate::gamestates::player_data::PlayerData;
use crate::gamestates::physics_object::PhysicsObject;

// #[derive(Default)]
#[derive(Clone)]
pub struct GameState {
    pub game_type: i32,
    pub blue_score: i32,
    pub orange_score: i32,
    pub last_touch: i32,
    pub players: Vec<PlayerData>,
    pub ball: PhysicsObject,
    pub inverted_ball: PhysicsObject,
    pub boost_pads: Vec<f32>,
    pub inverted_boost_pads: Vec<f32>
}

const BOOST_PAD_LENGTH: usize = 34;
const BALL_STATE_LENGTH: usize = 18;
const PLAYER_CAR_STATE_LENGTH: usize = 13;
const PLAYER_TERTIARY_INFO_LENGTH: usize = 11;
const PLAYER_INFO_LENGTH: usize = 2 + 2 * PLAYER_CAR_STATE_LENGTH + PLAYER_TERTIARY_INFO_LENGTH;

impl GameState {
    pub fn new(state_floats: Option<Vec<f32>>) -> Self {
        let game_st = match state_floats {
            Some(state_floats) => {                
                let mut game_st = GameState {
                    game_type: 0,
                    blue_score: -1,
                    orange_score: -1,
                    last_touch: -1,
                    players: Vec::<PlayerData>::new(),
                    ball: PhysicsObject::new(),
                    inverted_ball: PhysicsObject::new(),
                    boost_pads: Vec::<f32>::new(),
                    inverted_boost_pads: Vec::<f32>::new()
                };
                game_st.decode(state_floats);
                return game_st
            }
            None => {
                GameState {
                    game_type: 0,
                    blue_score: -1,
                    orange_score: -1,
                    last_touch: -1,
                    players: Vec::<PlayerData>::new(),
                    ball: PhysicsObject::new(),
                    inverted_ball: PhysicsObject::new(),
                    boost_pads: Vec::<f32>::new(),
                    inverted_boost_pads: Vec::<f32>::new()
                }
            }
        };
        // Default::default()
        // GameState {
        //     game_type: 0,
        //     blue_score: -1,
        //     orange_score: -1,
        //     last_touch: -1,
        //     players: Vec::<PlayerData>::new(),
        //     ball: PhysicsObject::new(),
        //     inverted_ball: PhysicsObject::new(),
        //     boost_pads: Vec::<f32>::new(),
        //     inverted_boost_pads: Vec::<f32>::new()
        // }
        return game_st
    }

    pub fn decode(&mut self, state_vals: Vec<f32>) {
        let mut start = 3;
        let num_ball_packets = 1;
        let state_val_len = state_vals.len();

        let num_player_packets = (state_val_len - num_ball_packets * BALL_STATE_LENGTH - start - BOOST_PAD_LENGTH) / PLAYER_INFO_LENGTH;

        self.blue_score = state_vals[1] as i32;
        self.orange_score = state_vals[2] as i32;

        self.boost_pads = state_vals[start..start+BOOST_PAD_LENGTH].to_vec();
        self.inverted_boost_pads = self.boost_pads.clone();
        self.inverted_boost_pads.reverse();
        start = start + BOOST_PAD_LENGTH;

        self.ball.decode_ball_data(state_vals[start..start+BALL_STATE_LENGTH].to_vec());
        start = start + (BALL_STATE_LENGTH / 2);

        self.inverted_ball.decode_ball_data(state_vals[start..start+BALL_STATE_LENGTH].to_vec());
        start = start + (BALL_STATE_LENGTH / 2);

        for _ in 0..num_player_packets {
            let player = self.decode_player(state_vals[start..start+PLAYER_INFO_LENGTH].to_vec());
            if player.ball_touched {
                self.last_touch = player.car_id as i32;
            }
            self.players.push(player);
            start = start + PLAYER_INFO_LENGTH;

        }

    }

    fn decode_player(&mut self, full_player_data: Vec<f32>) -> PlayerData {
        let mut player_data = PlayerData::new();

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
