use crate::gamestates::physics_object::PhysicsObject;

/// Struct which holds extra data for agents/players aside from just the PhysicsObjects
#[derive(Clone)]
pub struct PlayerData {
    pub car_id: i32,
    pub team_num: i32,
    pub match_goals: i64,
    pub match_saves: i64,
    pub match_shots: i64,
    pub match_demolishes: i64,
    pub boost_pickups: i64,
    pub is_demoed: bool,
    pub on_ground: bool,
    pub ball_touched: bool,
    pub has_jump: bool,
    pub has_flip: bool,
    pub boost_amount: f64,
    pub car_data: PhysicsObject,
    pub inverted_car_data: PhysicsObject
}

impl PlayerData {
    pub fn new() -> Self {
        // Default::default()
        PlayerData {
            car_id: -1,
            team_num: -1,
            match_goals: -1,
            match_saves: -1,
            match_shots: -1,
            match_demolishes: -1,
            boost_pickups: -1,
            is_demoed: false,
            on_ground: false,
            ball_touched: false,
            has_jump: false,
            has_flip: false,
            boost_amount: -1.,
            car_data: PhysicsObject::new(),
            inverted_car_data: PhysicsObject::new()
        }
    }
}