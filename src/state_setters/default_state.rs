use std::f64::consts::PI;
use rand::{rngs::StdRng, SeedableRng, thread_rng, Rng};

use super::{state_setter::StateSetter, wrappers::state_wrapper::StateWrapper};

pub struct DefaultState {
    spawn_blue_pos: Vec<Vec<f64>>,
    spawn_blue_yaw: Vec<f64>,
    spawn_orange_pos: Vec<Vec<f64>>,
    spawn_orange_yaw: Vec<f64>,
    rng: StdRng
}

impl DefaultState {
    pub fn new(seed: Option<u64>) -> Self {
        let seed = match seed {
            Some(seed) => seed,
            None => thread_rng().gen_range(0..10000)
        };
        let rng = StdRng::seed_from_u64(seed);
        DefaultState {
            spawn_blue_pos: vec![
                vec![-2048., -2560., 17.],
                vec![2048., -2560., 17.],
                vec![-256., -3840., 17.],
                vec![256., -3840., 17.],
                vec![0., -4608., 17.]
                ],
            spawn_blue_yaw: vec![0.25*PI, 0.75*PI, 0.5*PI, 0.5*PI, 0.5*PI],
            spawn_orange_pos: vec![
                vec![2048., 2560., 17.],
                vec![-2048., 2560., 17.],
                vec![256., 3840., 17.],
                vec![-256., 3840., 17.],
                vec![0., 4608., 17.]
                ],
            spawn_orange_yaw: vec![-0.75*PI, -0.25*PI, -0.5*PI, -0.5*PI, -0.5*PI],
            rng: rng
        }
    }
}

impl StateSetter for DefaultState {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let mut spawn_inds = vec![0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        spawn_inds.sort_unstable_by_key(|x| self.rng.gen::<usize>());

        let mut blue_count = 0;
        let mut orange_count = 0;
        for car in &mut state_wrapper.cars {
            let pos;
            let yaw: f64;

            if car.team_num == 0 {
                pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
                yaw = self.spawn_blue_yaw[spawn_inds[blue_count]].clone();
                blue_count += 1;
            } else {
                pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
                yaw = self.spawn_orange_yaw[spawn_inds[orange_count]].clone();
                orange_count += 1;
            }

            car.set_pos(Some(pos[0]), Some(pos[1]), Some(pos[2]));
            car.set_rot(None, Some(yaw), None);
            car.boost = 0.33;
        }
    }
    
    fn set_seed(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
    }
}

// this has no randomization for testing
pub struct DefaultStateTester {
    spawn_blue_pos: Vec<Vec<f64>>,
    spawn_blue_yaw: Vec<f64>,
    spawn_orange_pos: Vec<Vec<f64>>,
    spawn_orange_yaw: Vec<f64>
}

impl DefaultStateTester {
    pub fn new() -> Self {
        DefaultStateTester {
            spawn_blue_pos: vec![
                vec![-2048., -2560., 17.],
                vec![2048., -2560., 17.],
                vec![-256., -3840., 17.],
                vec![256., -3840., 17.],
                vec![0., -4608., 17.]
                ],
            spawn_blue_yaw: vec![0.25*PI, 0.75*PI, 0.5*PI, 0.5*PI, 0.5*PI],
            spawn_orange_pos: vec![
                vec![2048., 2560., 17.],
                vec![-2048., 2560., 17.],
                vec![256., 3840., 17.],
                vec![-256., 3840., 17.],
                vec![0., 4608., 17.]
                ],
            spawn_orange_yaw: vec![-0.75*PI, -0.25*PI, -0.5*PI, -0.5*PI, -0.5*PI]
        }
    }
}

impl StateSetter for DefaultStateTester {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let spawn_inds = vec![0, 1, 2, 3, 4];
        // let mut rng = rand::thread_rng();
        // spawn_inds.sort_by_key(|x| rng.gen::<usize>());

        let mut blue_count = 0;
        let mut orange_count = 0;
        for car in &mut state_wrapper.cars {
            let pos;
            let yaw: f64;

            if car.team_num == 0 {
                pos = self.spawn_blue_pos[spawn_inds[blue_count]].clone();
                yaw = self.spawn_blue_yaw[spawn_inds[blue_count]].clone();
                blue_count += 1;
            } else {
                pos = self.spawn_orange_pos[spawn_inds[orange_count]].clone();
                yaw = self.spawn_orange_yaw[spawn_inds[orange_count]].clone();
                orange_count += 1;
            }

            car.set_pos(Some(pos[0]), Some(pos[1]), Some(pos[2]));
            car.set_rot(None, Some(yaw), None);
            car.boost = 0.33;
        }
    }
}