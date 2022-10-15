use std::f64::consts::PI;
use rand::prelude::*;

use crate::math::rand_vec3;

use super::{wrappers::state_wrapper::StateWrapper, state_setter::StateSetter};

const X_MAX: f64 = 7000.;
const Y_MAX: f64 = 9000.;
const Z_MAX_BALL: f64 = 1850.;
const Z_MAX_CAR: f64 = 1900.;
const PITCH_MAX: f64 = PI/2.;
const YAW_MAX: f64 = PI;
const ROLL_MAX: f64 = PI;

/// Random state setter that makes random position/velocity/rotation values for each car and for the ball (within reason, eg. below max speeds)
pub struct RandomState {
    ball_rand_speed: bool,
    cars_rand_speed: bool,
    cars_on_ground: bool,
    rng: StdRng
}

impl RandomState {
    pub fn new(ball_rand_speed: Option<bool>, cars_rand_speed: Option<bool>, cars_on_ground: Option<bool>, seed: Option<u64>) -> Self {
        let ball_rand_speed = match ball_rand_speed {
            Some(ball_rand_speed) => ball_rand_speed,
            None => false
        };
        let cars_rand_speed = match cars_rand_speed {
            Some(cars_rand_speed) => cars_rand_speed,
            None => false
        };
        let cars_on_ground = match cars_on_ground {
            Some(cars_on_ground) => cars_on_ground,
            None => false
        };
        let seed = match seed {
            Some(seed) => seed,
            None => thread_rng().gen_range(0..10000)
        };
        let rng = StdRng::seed_from_u64(seed);

        RandomState {
            ball_rand_speed,
            cars_rand_speed,
            cars_on_ground,
            rng
        }
    }

    // pub fn reset(&self, state_wrapper: StateWrapper) {
    //     self._reset_ball_random(state_wrapper, self.ball_rand_speed);
    //     self._reset_cars_random(state_wrapper, self.cars_on_ground, self.cars_rand_speed);
    // }

    fn _reset_ball_random(&mut self, state_wrapper: &mut StateWrapper, random_speed: bool) {
        // let mut rng  = rand::thread_rng();
        state_wrapper.ball.set_pos(Some(self.rng.gen::<f64>() * X_MAX - X_MAX/2.), Some(self.rng.gen::<f64>() * Y_MAX - Y_MAX/2.), Some(self.rng.gen::<f64>() * Z_MAX_BALL + 100.));
        if random_speed {
            let lin_vel = rand_vec3(3000., &mut self.rng);
            let ang_vel = rand_vec3(6., &mut self.rng);
            state_wrapper.ball.set_lin_vel(Some(lin_vel[0]), Some(lin_vel[1]), Some(lin_vel[2]));
            state_wrapper.ball.set_ang_vel(Some(ang_vel[0]), Some(ang_vel[1]), Some(ang_vel[2]));
        }
    }

    fn _reset_cars_random(&mut self, state_wrapper: &mut StateWrapper, on_ground: bool, random_speed: bool) {
        // let mut rng  = rand::thread_rng();
        // let cars = &mut state_wrapper.cars;
        for mut car in &mut state_wrapper.cars {
            car.set_pos(Some(self.rng.gen::<f64>() * X_MAX - X_MAX/2.), Some(self.rng.gen::<f64>()
            * Y_MAX - Y_MAX/2.), Some(self.rng.gen::<f64>() * Z_MAX_CAR + 150.));
            car.set_rot(Some(self.rng.gen::<f64>() * PITCH_MAX - PITCH_MAX/2.), Some(self.rng.gen::<f64>()
            * YAW_MAX - YAW_MAX/2.), Some(self.rng.gen::<f64>() * ROLL_MAX - ROLL_MAX/2.));
            
            car.boost = self.rng.gen::<f64>();

            if random_speed {
                let lin_vel = rand_vec3(2300., &mut self.rng);
                let ang_vel = rand_vec3(5.5, &mut self.rng);
                car.set_lin_vel(Some(lin_vel[0]), Some(lin_vel[1]), Some(lin_vel[2]));
                car.set_ang_vel(Some(ang_vel[0]), Some(ang_vel[1]), Some(ang_vel[2]));
            }

            if on_ground || self.rng.gen::<f64>() < 0.5 {
                car.set_pos(None, None, Some(17.));
                car.set_lin_vel(None, None, Some(0.));
                car.set_rot(Some(0.), None, Some(0.));
                car.set_ang_vel(Some(0.), Some(0.), None);
            }
        }
    }
}

impl StateSetter for RandomState {
    fn reset(&mut self, mut state_wrapper: &mut StateWrapper) {
        self._reset_ball_random(&mut state_wrapper, self.ball_rand_speed);
        self._reset_cars_random(&mut state_wrapper, self.cars_on_ground, self.cars_rand_speed);
    }

    fn set_seed(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
    }
}

