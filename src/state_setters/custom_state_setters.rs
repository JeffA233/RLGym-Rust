use super::default_state::DefaultState;
use super::random_state::RandomState;
use super::state_setter::StateSetter;
use super::wrappers::state_wrapper::StateWrapper;
use rand::Rng;
use rand::distributions::weighted::WeightedIndex;
use rand::prelude::Distribution;
use rand::{rngs::StdRng, SeedableRng, thread_rng};

use zip::read::ZipArchive;
use std::{fs::File, io::BufReader};
use serde_json::from_reader;


pub fn custom_state_setters(team_size: i32, seed: Option<u64>) -> WeightedSampleSetter {
    let replay_setter_str = if team_size == 1 {"replay_folder/ssl_1v1.zip".to_owned()} else if team_size == 2 {"replay_folder/ssl_2v2.zip".to_owned()} else {"replay_folder/ssl_3v3.zip".to_owned()};
    let state_setters: Vec<Box<dyn StateSetter + Send>> = vec![
        Box::new(DefaultState::new(seed)),
        Box::new(RandomState::new(None, None, Some(false), seed)),
        Box::new(ReplaySetter::new(replay_setter_str))
    ];
    WeightedSampleSetter::new(state_setters, vec![1.0, 0.15, 0.5], seed)
}

pub struct WeightedSampleSetter {
    state_setters: Vec<Box<dyn StateSetter + Send>>,
    distribution: WeightedIndex<f64>,
    rng: StdRng
}

impl WeightedSampleSetter {
    pub fn new(state_setters: Vec<Box<dyn StateSetter + Send>>, weights: Vec<f64>, seed: Option<u64>) -> Self {
        assert!(state_setters.len() == weights.len(), "WeightedSampleSetter requires the argument lengths match");
        let distribution =  WeightedIndex::new(&weights).unwrap();
        let seed = match seed {
            Some(seed) => seed,
            None => thread_rng().gen_range(0..10000)
        };
        let rng = StdRng::seed_from_u64(seed);
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

    fn set_seed(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
        for state_setter in &mut self.state_setters {
            state_setter.set_seed(seed);
        }
    }
}

pub struct ReplaySetter {
    states: Vec<Vec<f64>>,
    rng: StdRng
}

impl ReplaySetter {
    pub fn new(file_str: String) -> Self {
        let file = File::open(file_str).unwrap();
        let mut zip_file = ZipArchive::new(file).unwrap();
        let zip_file = zip_file.by_index(0).unwrap();
        let reader = BufReader::new(zip_file);
        let states: Result<Vec<Vec<f64>>, serde_json::Error> = from_reader(reader);
        let states = match states {
            Ok(values) => values,
            Err(values) => panic!("{values}")
        };

        let seed = thread_rng().gen_range(0..10000);
        let rng = StdRng::seed_from_u64(seed);
        // rng.gen_range(0..states.len());
        ReplaySetter {
            states,
            rng
        }
    }

    fn _set_cars(state_wrapper: &mut StateWrapper, state: &mut Vec<f64>) {
        let data = &mut state[9..state_wrapper.cars.len()*13+9];
        let mut i = 0;
        for car in state_wrapper.cars.iter_mut() {
            car.set_pos(Some(data[i]), Some(data[i+1]), Some(data[i+2]));
            car.set_rot(Some(data[i+3]), Some(data[i+4]), Some(data[i+5]));
            car.set_lin_vel(Some(data[i+6]), Some(data[i+7]), Some(data[i+8]));
            car.set_ang_vel(Some(data[i+9]), Some(data[i+10]), Some(data[i+11]));
            car.boost = data[i+12];
            i += 13;
        }
    }

    fn _set_ball(state_wrapper: &mut StateWrapper, data: &mut Vec<f64>) {
        state_wrapper.ball.set_pos(Some(data[0]), Some(data[1]), Some(data[2]));
        state_wrapper.ball.set_lin_vel(Some(data[3]), Some(data[4]), Some(data[5]));
        state_wrapper.ball.set_lin_vel(Some(data[6]), Some(data[7]), Some(data[8]));
    }
}

impl StateSetter for ReplaySetter {
    fn reset(&mut self, state_wrapper: &mut StateWrapper) {
        let mut state = self.states[self.rng.gen_range(0..self.states.len())].clone();
        ReplaySetter::_set_ball(state_wrapper, &mut state);
        ReplaySetter::_set_cars(state_wrapper, &mut state);
    }
}