use std::f32::consts::PI;
use rand::prelude::*;

use crate::math::rand_vec3;

use super::{wrappers::state_wrapper::StateWrapper, state_setter::StateSetter};

const X_MAX: f32 = 7000.;
const Y_MAX: f32 = 9000.;
const Z_MAX_BALL: f32 = 1850.;
const Z_MAX_CAR: f32 = 1900.;
const PITCH_MAX: f32 = PI/2.;
const YAW_MAX: f32 = PI;
const ROLL_MAX: f32 = PI;

pub struct RandomState {
    ball_rand_speed: bool,
    cars_rand_speed: bool,
    cars_on_ground: bool
}

impl RandomState {
    pub fn new(ball_rand_speed: Option<bool>, cars_rand_speed: Option<bool>, cars_on_ground: Option<bool>) -> Self {
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

        RandomState {
            ball_rand_speed,
            cars_rand_speed,
            cars_on_ground
        }
    }

    // pub fn reset(&self, state_wrapper: StateWrapper) {
    //     self._reset_ball_random(state_wrapper, self.ball_rand_speed);
    //     self._reset_cars_random(state_wrapper, self.cars_on_ground, self.cars_rand_speed);
    // }

    fn _reset_ball_random(&self, state_wrapper: &mut StateWrapper, random_speed: bool) {
        let mut rng  = rand::thread_rng();
        state_wrapper.ball.set_pos(Some(rng.gen::<f32>() * X_MAX - X_MAX/2.), Some(rng.gen::<f32>() * Y_MAX - Y_MAX/2.), Some(rng.gen::<f32>() * Z_MAX_BALL + 100.));
        if random_speed {
            let lin_vel = rand_vec3(3000.);
            let ang_vel = rand_vec3(6.);
            state_wrapper.ball.set_lin_vel(Some(lin_vel[0]), Some(lin_vel[1]), Some(lin_vel[2]));
            state_wrapper.ball.set_ang_vel(Some(ang_vel[0]), Some(ang_vel[1]), Some(ang_vel[2]));
        }
    }

    fn _reset_cars_random(&self, state_wrapper: &mut StateWrapper, on_ground: bool, random_speed: bool) {
        let mut rng  = rand::thread_rng();
        // let cars = &mut state_wrapper.cars;
        for mut car in &mut state_wrapper.cars {
            car.set_pos(Some(rng.gen::<f32>() * X_MAX - X_MAX/2.), Some(rng.gen::<f32>()
            * Y_MAX - Y_MAX/2.), Some(rng.gen::<f32>() * Z_MAX_CAR + 150.));
            car.set_rot(Some(rng.gen::<f32>() * PITCH_MAX - PITCH_MAX/2.), Some(rng.gen::<f32>()
            * YAW_MAX - YAW_MAX/2.), Some(rng.gen::<f32>() * ROLL_MAX - ROLL_MAX/2.));
            
            car.boost = rng.gen::<f32>();

            if random_speed {
                let lin_vel = rand_vec3(2300.);
                let ang_vel = rand_vec3(5.5);
                car.set_lin_vel(Some(lin_vel[0]), Some(lin_vel[1]), Some(lin_vel[2]));
                car.set_ang_vel(Some(ang_vel[0]), Some(ang_vel[1]), Some(ang_vel[2]));
            }

            if on_ground || rng.gen::<f32>() < 0.5 {
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

    // fn build_wrapper(&mut self, max_team_size: i32, spawn_opponents: bool) -> StateWrapper {
    //     StateWrapper::new(Some(max_team_size), if spawn_opponents {Some(max_team_size)} else {Some(0)}, None)
    // }
}

