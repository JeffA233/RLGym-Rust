// use core::num;

use crate::common_values::BLUE_TEAM;
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
    pub boost_pads: Vec<f64>,
    pub inverted_boost_pads: Vec<f64>
}

const BOOST_PAD_LENGTH: usize = 34;
const BALL_STATE_LENGTH: usize = 18;
const PLAYER_CAR_STATE_LENGTH: usize = 13;
const PLAYER_TERTIARY_INFO_LENGTH: usize = 11;
const PLAYER_INFO_LENGTH: usize = 2 + 2 * PLAYER_CAR_STATE_LENGTH + PLAYER_TERTIARY_INFO_LENGTH;

impl GameState {
    pub fn new(state_floats: Option<Vec<f64>>) -> Self {
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
                    boost_pads: Vec::<f64>::new(),
                    inverted_boost_pads: Vec::<f64>::new()
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
                    boost_pads: Vec::<f64>::new(),
                    inverted_boost_pads: Vec::<f64>::new()
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

    pub fn decode(&mut self, state_vals: Vec<f64>) {
        let mut start = 3;
        let num_ball_packets = 1;
        let state_val_len = state_vals.len();

        let num_player_packets = (state_val_len as i32 - num_ball_packets as i32 * BALL_STATE_LENGTH as i32 - start as i32 - BOOST_PAD_LENGTH as i32) / PLAYER_INFO_LENGTH  as i32;

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
                self.last_touch = player.car_id.clone();
            }
            self.players.push(player);
            start = start + PLAYER_INFO_LENGTH;

        }
        self.players.sort_unstable_by_key(|p| p.car_id);
    }

    fn decode_player(&self, full_player_data: Vec<f64>) -> PlayerData {
        let mut player_data = PlayerData::new();

        let mut start: usize = 2;

        player_data.car_data.decode_car_data(full_player_data[start..start+PLAYER_CAR_STATE_LENGTH].to_vec());
        start = start + PLAYER_CAR_STATE_LENGTH;

        player_data.inverted_car_data.decode_car_data(full_player_data[start..start+PLAYER_CAR_STATE_LENGTH].to_vec());
        start = start + PLAYER_CAR_STATE_LENGTH;

        let tertiary_data = &full_player_data[start..start+PLAYER_TERTIARY_INFO_LENGTH];

        player_data.match_goals = tertiary_data[0] as i64;
        player_data.match_saves = tertiary_data[1] as i64;
        player_data.match_shots = tertiary_data[2] as i64;
        player_data.match_demolishes = tertiary_data[3] as i64;
        player_data.boost_pickups = tertiary_data[4] as i64;
        player_data.is_demoed = tertiary_data[5] > 0.;
        player_data.on_ground = tertiary_data[6] > 0.;
        player_data.ball_touched = tertiary_data[7] > 0.;
        player_data.has_jump = tertiary_data[8] > 0.;
        player_data.has_flip = tertiary_data[9] > 0.;
        player_data.boost_amount = tertiary_data[10];
        player_data.car_id = full_player_data[0] as i32;
        player_data.team_num = full_player_data[1] as i32;
        
        return player_data
    }
}

// #[derive(Clone)]
// pub struct FakeGameState {
//     pub game_type: i32,
//     pub blue_score: i32,
//     pub orange_score: i32,
//     pub last_touch: i32,
//     pub players: Vec<PlayerData>,
//     pub ball: PhysicsObject,
//     pub inverted_ball: PhysicsObject,
//     pub boost_pads: Vec<f64>,
//     pub inverted_boost_pads: Vec<f64>
// }


impl GameState {
    pub fn new_test() -> Self {    
        let mut ball = PhysicsObject::new();
        ball.position = vec![300., 300., 92.75];
        ball.linear_velocity = vec![100., 5., 10.];
        ball.angular_velocity = vec![75., -2., 5.];
        let mut car = PhysicsObject::new();
        car.position = vec![0., 0., 17.0];
        car.linear_velocity = vec![-5., -3., 0.];
        car.angular_velocity = vec![-3., -1., 0.1];
        let mut car2 = PhysicsObject::new();
        car2.position = vec![50., 0., 17.0];
        car2.linear_velocity = vec![-5., -3., 0.];
        car2.angular_velocity = vec![-3., -1., 0.1];
        GameState {
            game_type: 0,
            blue_score: 0,
            orange_score: 0,
            last_touch: 0,
            players: vec![
                PlayerData {
                    car_id: 1,
                    team_num: BLUE_TEAM,
                    match_goals: 0,
                    match_saves: 0,
                    match_shots: 0,
                    match_demolishes: 0,
                    boost_pickups: 0,
                    is_demoed: false,
                    boost_amount: 0.34,
                    on_ground: true,
                    ball_touched: false,
                    has_flip: true,
                    has_jump: true,
                    car_data: car,
                    inverted_car_data: PhysicsObject::new()
                },
                PlayerData {
                    car_id: 2,
                    team_num: BLUE_TEAM,
                    match_goals: 0,
                    match_saves: 0,
                    match_shots: 0,
                    match_demolishes: 0,
                    boost_pickups: 0,
                    is_demoed: false,
                    boost_amount: 0.34,
                    on_ground: true,
                    ball_touched: false,
                    has_flip: true,
                    has_jump: true,
                    car_data: car2,
                    inverted_car_data: PhysicsObject::new()
                }
            ],
            ball: ball,
            inverted_ball: PhysicsObject::new(),
            boost_pads: vec![0.; 34],
            inverted_boost_pads: vec![0.; 34]
        }
    }

    // pub fn decode(&mut self, state_vals: Vec<f64>) {
    //     let mut start = 3;
    //     let num_ball_packets = 1;
    //     let state_val_len = state_vals.len();

    //     let num_player_packets = (state_val_len as i32 - num_ball_packets as i32 * BALL_STATE_LENGTH as i32 - start as i32 - BOOST_PAD_LENGTH as i32) / PLAYER_INFO_LENGTH  as i32;

    //     self.blue_score = state_vals[1] as i32;
    //     self.orange_score = state_vals[2] as i32;

    //     self.boost_pads = state_vals[start..start+BOOST_PAD_LENGTH].to_vec();
    //     self.inverted_boost_pads = self.boost_pads.clone();
    //     self.inverted_boost_pads.reverse();
    //     start = start + BOOST_PAD_LENGTH;

    //     self.ball.decode_ball_data(state_vals[start..start+BALL_STATE_LENGTH].to_vec());
    //     start = start + (BALL_STATE_LENGTH / 2);

    //     self.inverted_ball.decode_ball_data(state_vals[start..start+BALL_STATE_LENGTH].to_vec());
    //     start = start + (BALL_STATE_LENGTH / 2);

    //     for _ in 0..num_player_packets {
    //         let player = self.decode_player(state_vals[start..start+PLAYER_INFO_LENGTH].to_vec());
    //         if player.ball_touched {
    //             self.last_touch = player.car_id as i32;
    //         }
    //         self.players.push(player);
    //         start = start + PLAYER_INFO_LENGTH;

    //     }
    //     self.players.sort_unstable_by_key(|p| p.car_id);
    // }

    // fn decode_player(&self, full_player_data: Vec<f64>) -> PlayerData {
    //     let mut player_data = PlayerData::new();

    //     let mut start: usize = 2;

    //     player_data.car_data.decode_car_data(full_player_data[start..start+PLAYER_CAR_STATE_LENGTH].to_vec());
    //     start = start + PLAYER_CAR_STATE_LENGTH;

    //     player_data.inverted_car_data.decode_car_data(full_player_data[start..start+PLAYER_CAR_STATE_LENGTH].to_vec());
    //     start = start + PLAYER_CAR_STATE_LENGTH;

    //     let tertiary_data = &full_player_data[start..start+PLAYER_TERTIARY_INFO_LENGTH];

    //     player_data.match_goals = tertiary_data[0] as i64;
    //     player_data.match_saves = tertiary_data[1] as i64;
    //     player_data.match_shots = tertiary_data[2] as i64;
    //     player_data.match_demolishes = tertiary_data[3] as i64;
    //     player_data.boost_pickups = tertiary_data[4] as i64;
    //     player_data.is_demoed = tertiary_data[5] > 0.;
    //     player_data.on_ground = tertiary_data[6] > 0.;
    //     player_data.ball_touched = tertiary_data[7] > 0.;
    //     player_data.has_jump = tertiary_data[8] > 0.;
    //     player_data.has_flip = tertiary_data[9] > 0.;
    //     player_data.boost_amount = tertiary_data[10];
    //     player_data.car_id = full_player_data[0] as i32;
    //     player_data.team_num = full_player_data[1] as i32;
        
    //     return player_data
    // }
}
