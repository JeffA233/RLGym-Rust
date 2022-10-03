pub mod continuous_action {
    use gym::*;
    // use ndarray::*;
    use crate::math::clip;

    pub fn get_actions() -> Vec<usize> {
        vec![0, 0, 0, 0, 0, 0, 0, 0]
    }

    pub fn get_action_space() -> SpaceTemplate {
        // let mut vec: Vec<usize> = Vec::new();
        // vec.resize(get_num_actions(), 0);
        // let mut vec_low: Vec<f64> = Vec::new();
        // vec_low.resize(get_num_actions(), -1.);
        // let mut vec_high: Vec<f64> = Vec::new();
        // vec_high.resize(get_num_actions(), 1.);
        let vec: Vec<usize> = get_actions();
        let vec_low: Vec<f64> = vec![-1., -1., -1., -1., -1., -1., -1., -1.];
        let vec_high: Vec<f64> = vec![1., 1., 1., 1., 1., 1., 1., 1.];
        return SpaceTemplate::BOX { high: vec_high, low: vec_low, shape: vec };
    }

    pub fn parse_actions(mut actions: Vec<f64>) -> Vec<f64> {
        // let mut actions = actions.clone();
        // let mut clipped_actions = &actions[0 as usize..5];
        actions = clip(actions, 1., -1.);
        // sadly we cannot 
        // actions.slice_mut(s![5 as usize..]).map(|x| x > &0.);
        // clipped_actions.into_iter();
        // for i in 5..=8 {
        //     let mut val = clipped_actions[i];
        //     if val > 0. {
        //         clipped_actions[i] = true;
        //     }

        // }
        
        return actions;
    }
}