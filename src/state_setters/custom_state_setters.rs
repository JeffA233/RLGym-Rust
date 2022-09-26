use super::state_setter::StateSetter;
use super::wrappers::state_wrapper::StateWrapper;
use rand::Rng;
use rand::distributions::weighted::WeightedIndex;
use rand::prelude::{thread_rng, Distribution};
use rand::rngs::ThreadRng;

use zip::read::ZipArchive;
use std::{fs::File, io::BufReader};
use serde_json::from_reader;


pub struct WeightedSampleSetter {
    state_setters: Vec<Box<dyn StateSetter + Send>>,
    distribution: WeightedIndex<f32>,
    rng: ThreadRng
}

impl WeightedSampleSetter {
    pub fn new(state_setters: Vec<Box<dyn StateSetter + Send>>, weights: Vec<f32>) -> Self {
        assert!(state_setters.len() == weights.len(), "WeightedSampleSetter requires the argument lengths match");
        let distribution =  WeightedIndex::new(&weights).unwrap();
        let rng = thread_rng();
        WeightedSampleSetter {
            state_setters,
            distribution,
            rng
        }
    }
}

impl StateSetter for WeightedSampleSetter {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let choice = self.distribution.sample(&mut self.rng);
        self.state_setters[choice].reset(state_wrapper);
    }
}

pub struct ReplaySetter {
    states: Vec<Vec<f32>>,
    rng: ThreadRng
}

impl ReplaySetter {
    pub fn new(file_str: String) -> Self {
        let file = File::open(file_str).unwrap();
        let mut zip_file = ZipArchive::new(file).unwrap();
        let zip_file = zip_file.by_index(0).unwrap();
        let reader = BufReader::new(zip_file);
        let states: Result<Vec<Vec<f32>>, serde_json::Error> = from_reader(reader);
        let states = match states {
            Ok(values) => values,
            Err(values) => panic!("{values}")
        };

        let rng = thread_rng();
        // rng.gen_range(0..states.len());
        ReplaySetter {
            states,
            rng
        }
    }

    fn _set_cars(state_wrapper: &mut StateWrapper, state: &mut Vec<f32>) {
        let data = &mut state[9..state_wrapper.cars.len()];
        let mut i = 0;
        for car in state_wrapper.cars.iter_mut() {
            car.set_pos(Some(data[i+0]), Some(data[i+1]), Some(data[i+2]));
            car.set_rot(Some(data[i+3]), Some(data[i+4]), Some(data[i+5]));
            car.set_lin_vel(Some(data[i+6]), Some(data[i+7]), Some(data[i+8]));
            car.set_ang_vel(Some(data[i+9]), Some(data[i+10]), Some(data[i+11]));
            car.boost = data[i+12];
            i += 13;
        }
    }

    fn _set_ball(state_wrapper: &mut StateWrapper, data: &mut Vec<f32>) {
        state_wrapper.ball.set_pos(Some(data[0]), Some(data[1]), Some(data[2]));
        state_wrapper.ball.set_lin_vel(Some(data[3]), Some(data[4]), Some(data[5]));
        state_wrapper.ball.set_lin_vel(Some(data[6]), Some(data[7]), Some(data[8]));
    }
}

impl StateSetter for ReplaySetter {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let state = self.states[self.rng.gen_range(0..self.states.len())].clone();

    }
}