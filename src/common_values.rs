const SIDE_WALL_X: usize = 4096;
const BACK_WALL_Y: usize = 5120;
const CEILING_Z: usize = 2044;
const BACK_NET_Y: usize = 6000;
const GOAL_HEIGHT: f64 = 642.775;

const ORANGE_GOAL_CENTER: [f64; 3] = [0., BACK_NET_Y as f64, GOAL_HEIGHT as f64 / 2.];
const BLUE_GOAL_CENTER: [f64; 3] = [0., -(BACK_NET_Y as f64), GOAL_HEIGHT as f64 / 2.];

const ORANGE_GOAL_BACK: [f64; 3] = [0., BACK_NET_Y as f64, GOAL_HEIGHT as f64 / 2.];
const BLUE_GOAL_BACK: [f64; 3] = [0., -(BACK_NET_Y as f64), GOAL_HEIGHT as f64 / 2.];

const BALL_RADIUS: f64 = 92.75;
const BALL_MAX_SPEED: f64 = 6000.0;

const CAR_MAX_SPEED: f64 = 2300.0;
const SUPERSONIC_THRESHOLD: f64 = 2200.0;
const CAR_MAX_ANG_VEL: f64 = 5.5;

const BLUE_TEAM: usize = 0;
const ORANGE_TEAM: usize = 1;

const NUM_ACTIONS: usize = 8;

const BOOST_LOCATIONS: [[f64; 3]; 34] = [
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