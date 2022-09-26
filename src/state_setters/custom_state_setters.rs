use super::state_setter::StateSetter;
use super::wrappers::state_wrapper::StateWrapper;
use rand::Rng;
use rand::distributions::weighted::WeightedIndex;
use rand::prelude::{thread_rng, Distribution};
use rand::rngs::ThreadRng;


pub struct WeightedSampleSetter {
    state_setters: Vec<Box<dyn StateSetter + Send>>,
    distribution: WeightedIndex<f32>,
    rng: ThreadRng
}

impl WeightedSampleSetter {
    pub fn new(state_setters: Vec<Box<dyn StateSetter + Send>>, weights: Vec<f32>) -> Self {
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