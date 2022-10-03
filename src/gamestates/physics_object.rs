// use numpy::*;
use ndarray::*;
use crate::math::*;
#[derive(Default)]
#[derive(Clone)]
pub struct PhysicsObject {
    pub position: Vec<f32>,
    pub quaternion: Vec<f32>,
    pub linear_velocity: Vec<f32>,
    pub angular_velocity: Vec<f32>,
    pub euler_angles: Vec<f32>,
    pub rotation_mtx: Array2<f32>,
    pub has_computed_rot_mtx: bool,
    pub has_computed_euler_angles: bool
}

impl PhysicsObject {
    pub fn new() -> Self {
        PhysicsObject {
            position: vec![0.; 3],
            quaternion: vec![0.; 4],
            linear_velocity: vec![0.; 3],
            angular_velocity: vec![0.; 3],
            euler_angles: vec![0.; 3],
            rotation_mtx: Array2::<f32>::zeros((3, 3)),
            has_computed_euler_angles: false,
            has_computed_rot_mtx: false
        }
        // Default::default()
    }

    // pub fn clone(&self) -> Self {
    //     PhysicsObject {
    //         position: self.position.clone(),
    //         quaternion: self.quaternion.clone(),
    //         linear_velocity: self.linear_velocity.clone(),
    //         angular_velocity: self.angular_velocity.clone(),
    //         euler_angles: self.euler_angles.clone(),
    //         rotation_mtx: self.rotation_mtx.clone(),
    //         has_computed_euler_angles: self.has_computed_euler_angles.clone(),
    //         has_computed_rot_mtx: self.has_computed_rot_mtx.clone()
    //     }
    // }

    pub fn decode_car_data(&mut self, car_data: Vec<f32>) {
        self.position = car_data[..3].to_vec();
        self.quaternion = car_data[3..7].to_vec();
        self.linear_velocity = car_data[7..10].to_vec();
        self.angular_velocity = car_data[10..].to_vec();
    }

    pub fn decode_ball_data(&mut self, ball_data: Vec<f32>) {
        self.position = ball_data[..3].to_vec();
        self.linear_velocity = ball_data[3..6].to_vec();
        self.angular_velocity = ball_data[6..9].to_vec();
    }

    pub fn forward(&mut self) -> Vec<f32> {
        let arr = &self.rotation_mtx();
        let partial_arr = arr.column(0);
        // [:, 0]
        // let ret_arr = partial_arr.to_owned();
        return partial_arr.to_owned().to_vec()
    }

    pub fn right(&mut self) -> Vec<f32> {
        let arr = self.rotation_mtx();
        let partial_arr = arr.column(1);
        return partial_arr.to_owned().to_vec()
    }

    pub fn left(&mut self) -> Vec<f32> {
        let arr = self.rotation_mtx();
        let partial_arr = arr.column(1);
        let res_arr = partial_arr.to_owned() * -1.;
        return res_arr.to_vec()
    }

    pub fn up(&mut self) -> Vec<f32> {
        let arr = self.rotation_mtx();
        let partial_arr = arr.column(2);
        return partial_arr.to_owned().to_vec()
    }

    pub fn pitch(&mut self) -> f32 {
        self.euler_angles()[0]
    }

    pub fn yaw(&mut self) -> f32 {
        self.euler_angles()[1]
    }

    pub fn roll(&mut self) -> f32 {
        self.euler_angles()[2]
    }

    pub fn euler_angles(&mut self) -> Vec<f32> {
        if !self.has_computed_euler_angles {
            self.euler_angles = quat_to_euler(&self.quaternion.to_vec());
            self.has_computed_euler_angles = true;
        }
        return self.euler_angles.clone()
    }
    
    pub fn rotation_mtx(&mut self) -> Array2<f32> {
        if !self.has_computed_rot_mtx {
            self.rotation_mtx = quat_to_rot_mtx(&self.quaternion.to_vec());
            self.has_computed_rot_mtx = true;
        }
        // let rotation_mtx = self.rotation_mtx.clone();
        return self.rotation_mtx.clone()
    }

    pub fn serialize(&self) -> Vec<f32> {
        let mut repr = Vec::<f32>::new();

        // repr.extend([&mut self.position, &mut self.quaternion.clone()]);

        repr.append(&mut self.position.clone());
        repr.append(&mut self.quaternion.clone());
        repr.append(&mut self.linear_velocity.clone());
        repr.append(&mut self.angular_velocity.clone());
        repr.append(&mut self.euler_angles.clone());
        
        let mut row_vec = Vec::<f32>::new();
        for i in self.rotation_mtx.clone() {
            row_vec.push(i)
        }
        repr.append(&mut row_vec);

        return repr
    }
}
