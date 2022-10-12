use crate::gamestates::physics_object::Position;

pub const SIDE_WALL_X: f64 = 4096.;
pub const BACK_WALL_Y: f64 = 5120.;
pub const CEILING_Z: f64 = 2044.;
pub const BACK_NET_Y: f64 = 6000.;
pub const GOAL_HEIGHT: f64 = 642.775;

// pub const ORANGE_GOAL_CENTER: [f64; 3] = [0., BACK_NET_Y as f64, GOAL_HEIGHT as f64 / 2.];
// pub const BLUE_GOAL_CENTER: [f64; 3] = [0., -(BACK_NET_Y as f64), GOAL_HEIGHT as f64 / 2.];
pub const ORANGE_GOAL_CENTER: Position = Position {x: 0., y: BACK_WALL_Y, z: GOAL_HEIGHT / 2.};
pub const BLUE_GOAL_CENTER: Position = Position {x: 0., y: -BACK_WALL_Y, z: GOAL_HEIGHT / 2.};

// pub const ORANGE_GOAL_BACK: [f64; 3] = [0., BACK_NET_Y as f64, GOAL_HEIGHT as f64 / 2.];
// pub const BLUE_GOAL_BACK: [f64; 3] = [0., -(BACK_NET_Y as f64), GOAL_HEIGHT as f64 / 2.];
pub const ORANGE_GOAL_BACK: Position = Position {x: 0., y: BACK_NET_Y, z: GOAL_HEIGHT / 2.};
pub const BLUE_GOAL_BACK: Position = Position {x: 0., y: -BACK_NET_Y, z: GOAL_HEIGHT / 2.};

pub const BALL_RADIUS: f64 = 92.75;
pub const BALL_MAX_SPEED: f64 = 6000.0;

pub const CAR_MAX_SPEED: f64 = 2300.0;
pub const SUPERSONIC_THRESHOLD: f64 = 2200.0;
pub const CAR_MAX_ANG_VEL: f64 = 5.5;

pub const BLUE_TEAM: i32 = 0;
pub const ORANGE_TEAM: i32 = 1;

pub const NUM_ACTIONS: usize = 8;

pub const BOOST_LOCATIONS: [[f64; 3]; 34] = [
    [0.0, -4240.0, 70.0],
    [-1792.0, -4184.0, 70.0],
    [1792.0, -4184.0, 70.0],
    [-3072.0, -4096.0, 73.0],
    [3072.0, -4096.0, 73.0],
    [-940.0, -3308.0, 70.0],
    [940.0, -3308.0, 70.0],
    [0.0, -2816.0, 70.0],
    [-3584.0, -2484.0, 70.0],
    [3584.0, -2484.0, 70.0],
    [-1788.0, -2300.0, 70.0],
    [1788.0, -2300.0, 70.0],
    [-2048.0, -1036.0, 70.0],
    [0.0, -1024.0, 70.0],
    [2048.0, -1036.0, 70.0],
    [-3584.0, 0.0, 73.0],
    [-1024.0, 0.0, 70.0],
    [1024.0, 0.0, 70.0],
    [3584.0, 0.0, 73.0],
    [-2048.0, 1036.0, 70.0],
    [0.0, 1024.0, 70.0],
    [2048.0, 1036.0, 70.0],
    [-1788.0, 2300.0, 70.0],
    [1788.0, 2300.0, 70.0],
    [-3584.0, 2484.0, 70.0],
    [3584.0, 2484.0, 70.0],
    [0.0, 2816.0, 70.0],
    [-940.0, 3310.0, 70.0],
    [940.0, 3308.0, 70.0],
    [-3072.0, 4096.0, 73.0],
    [3072.0, 4096.0, 73.0],
    [-1792.0, 4184.0, 70.0],
    [1792.0, 4184.0, 70.0],
    [0.0, 4240.0, 70.0]
];