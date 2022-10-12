use std::f64::consts::PI;

// use numpy::*;
use ndarray::*;


// start of helper structs

#[derive(Clone, Copy, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Position {
    pub fn set_vals(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
        match x {
            Some(val) => self.x = val,
            None => ()
        }
        match y {
            Some(val) => self.y = val,
            None => ()
        }
        match z {
            Some(val) => self.z = val,
            None => ()
        }
    }

    pub fn iter(&self) -> std::slice::Iter<f64> {
        return [self.x, self.y, self.z].iter()
    }

    pub fn to_vec(&self) -> Vec<f64> {
        return vec![self.x, self.y, self.z]
    }

    pub fn subtract(&self, other_vel: &Position) -> Position {
        let x = self.x - other_vel.x;
        let y = self.y - other_vel.y;
        let z = self.z - other_vel.z;
        return Position { x, y, z }
    }

    pub fn add(&self, other_vel: &Position) -> Position {
        let x = self.x + other_vel.x;
        let y = self.y + other_vel.y;
        let z = self.z + other_vel.z;
        return Position { x, y, z }
    }

    pub fn divide_by_var(&self, var: f64) -> Position {
        let x = self.x / var;
        let y = self.y / var;
        let z = self.z / var;
        return Position { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        let running_val = 0.;
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        return running_val.sqrt()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Velocity {
    pub fn set_vals(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
        match x {
            Some(val) => self.x = val,
            None => ()
        }
        match y {
            Some(val) => self.y = val,
            None => ()
        }
        match z {
            Some(val) => self.z = val,
            None => ()
        }
    }

    pub fn iter(&self) -> std::slice::Iter<f64> {
        return [self.x, self.y, self.z].iter()
    }

    pub fn to_vec(&self) -> Vec<f64> {
        return vec![self.x, self.y, self.z]
    }

    pub fn subtract(&self, other_vel: &Velocity) -> Velocity {
        let x = self.x - other_vel.x;
        let y = self.y - other_vel.y;
        let z = self.z - other_vel.z;
        return Velocity { x, y, z }
    }

    pub fn add(&self, other_vel: &Velocity) -> Velocity {
        let x = self.x + other_vel.x;
        let y = self.y + other_vel.y;
        let z = self.z + other_vel.z;
        return Velocity { x, y, z }
    }

    pub fn multiply(&self, other_vel: &Velocity) -> Velocity {
        let x = self.x * other_vel.x;
        let y = self.y * other_vel.y;
        let z = self.z * other_vel.z;
        return Velocity { x, y, z }
    }

    pub fn multiply_by_pos(&self, other_pos: &Position) -> Velocity {
        let x = self.x * other_pos.x;
        let y = self.y * other_pos.y;
        let z = self.z * other_pos.z;
        return Velocity { x, y, z }
    }

    pub fn divide_by_var(&self, var: f64) -> Velocity {
        let x = self.x / var;
        let y = self.y / var;
        let z = self.z / var;
        return Velocity { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        let running_val = 0.;
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        return running_val.sqrt()
    }

    pub fn scalar_projection(&self, dest_vec: &Position) -> f64 {
        // let norm = norm_func(&dest_vec);
        let norm = dest_vec.norm();
        if norm == 0. {
            return 0.;
        }
        return (self.multiply_by_pos(dest_vec).iter().sum::<f64>())/norm
        // return (element_mult_vec(&vec, &dest_vec).iter().sum::<f64>())/norm;
    }
}

#[derive(Clone, Copy, Default)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Quaternion {
    pub fn set_vals(&mut self, w: Option<f64>, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
        match w {
            Some(val) => self.w = val,
            None => ()
        }
        match x {
            Some(val) => self.x = val,
            None => ()
        }
        match y {
            Some(val) => self.y = val,
            None => ()
        }
        match z {
            Some(val) => self.z = val,
            None => ()
        }
    }
    
    pub fn norm(&self) -> f64 {
        let running_val = 0.;
        running_val += self.w.powi(2);
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        return running_val.sqrt()
    }

    /// quat Vec to rotation matrix Array2
    pub fn quat_to_rot_mtx(&self) -> Array2<f64> {
        let mut theta = Array2::<f64>::zeros((3, 3));
        
        // assert!(nums.len() == 4, "nums is not the correct shape");

        // let norm: f64 = nums.clone()
        //                     .into_iter()
        //                     .map(|x: f64| x.powf(2.))
        //                     // .collect::<Vec<f64>>()
        //                     // .iter()
        //                     .sum();

        let norm = self.norm();

        let w = -&self.w;
        let x = -&self.x;
        let y = -&self.y;
        let z = -&self.z;

        let s: f64 = 1.0 / norm;

        if norm != 0. {
            theta[[0, 0]] = 1. - 2. * s * (y * y + z * z);
            theta[[1, 0]] = 2. * s * (x * y + z * w);
            theta[[2, 0]] = 2. * s * (x * z - y * w);

            // left direction
            theta[[0, 1]] = 2. * s * (x * y - z * w);
            theta[[1, 1]] = 1. - 2. * s * (x * x + z * z);
            theta[[2, 1]] = 2. * s * (y * z + x * w);

            // up direction
            theta[[0, 2]] = 2. * s * (x * z + y * w);
            theta[[1, 2]] = 2. * s * (y * z - x * w);
            theta[[2, 2]] = 1. - 2. * s * (x * x + y * y);
        }

        return theta;
    }

    pub fn quat_to_euler(&self) -> EulerAngle{
        let w: f64 = self.w;
        let x: f64 = self.x;
        let y: f64 = self.y;
        let z: f64 = self.z;
    
        let sinr_cosp: f64 = 2. * (w * x + y * z);
        let cosr_cosp: f64 = 1. - 2. * (x * x + y * y);
        let sinp: f64 = 2. * (w * y + x * y);
        let siny_cosp: f64 = 2. * (w * z + x * y);
        let cosy_cosp: f64 = 1. - 2. * (y * y + z * z);
        let roll: f64 = sinr_cosp.atan2(cosr_cosp);
    
        let pitch: f64;
        if sinp.abs() > 1. {
            pitch = PI / 2.;
        }
        else {
            pitch = sinp.asin();
        }
    
        let yaw: f64 = siny_cosp.atan2(cosy_cosp);

        EulerAngle { x: -pitch, y: yaw, z: -roll }
    }

    pub fn iter(&self) -> std::slice::Iter<f64> {
        return [self.w, self.x, self.y, self.z].iter()
    }
}

#[derive(Clone, Copy, Default)]
pub struct EulerAngle {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl EulerAngle {
    pub fn set_vals(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
        match x {
            Some(val) => self.x = val,
            None => ()
        }
        match y {
            Some(val) => self.y = val,
            None => ()
        }
        match z {
            Some(val) => self.z = val,
            None => ()
        }
    }

    pub fn iter(&self) -> std::slice::Iter<f64> {
        return [self.x, self.y, self.z].iter()
    }
}

// end of helper structs
// -------------------------------------------------------------------------------------------
// start of PhysicsObject struct

#[derive(Default, Clone)]
pub struct PhysicsObject {
    pub position: Position,
    pub quaternion: Quaternion,
    pub linear_velocity: Velocity,
    pub angular_velocity: Velocity,
    pub euler_angles: EulerAngle,
    pub rotation_mtx: Array2<f64>,
    pub has_computed_rot_mtx: bool,
    pub has_computed_euler_angles: bool
}

impl PhysicsObject {
    // pub fn new() -> Self {
    //     PhysicsObject {
    //         position: vec![0.; 3],
    //         quaternion: vec![0.; 4],
    //         linear_velocity: vec![0.; 3],
    //         angular_velocity: vec![0.; 3],
    //         euler_angles: vec![0.; 3],
    //         rotation_mtx: Array2::<f64>::zeros((3, 3)),
    //         has_computed_euler_angles: false,
    //         has_computed_rot_mtx: false
    //     }
    // }
    pub fn new() -> Self {
        PhysicsObject {
            position: Position::default(),
            quaternion: Quaternion::default(),
            linear_velocity: Velocity::default(),
            angular_velocity: Velocity::default(),
            euler_angles: EulerAngle::default(),
            rotation_mtx: Array2::<f64>::zeros((3, 3)),
            has_computed_euler_angles: false,
            has_computed_rot_mtx: false
        }
    }

    pub fn decode_car_data(&mut self, car_data: Vec<f64>) {
        // self.position = car_data[..3].to_vec();
        // self.quaternion = car_data[3..7].to_vec();
        // self.linear_velocity = car_data[7..10].to_vec();
        // self.angular_velocity = car_data[10..].to_vec();
        self.position.set_vals(Some(car_data[0]), Some(car_data[1]), Some(car_data[2]));
        self.quaternion.set_vals(Some(car_data[3]), Some(car_data[4]), Some(car_data[5]), Some(car_data[6]));
        self.linear_velocity.set_vals(Some(car_data[7]), Some(car_data[8]), Some(car_data[9]));
        self.angular_velocity.set_vals(Some(car_data[10]), Some(car_data[11]), Some(car_data[12]));
    }

    pub fn decode_ball_data(&mut self, ball_data: Vec<f64>) {
        // self.position = ball_data[..3].to_vec();
        // self.linear_velocity = ball_data[3..6].to_vec();
        // self.angular_velocity = ball_data[6..9].to_vec();
        self.position.set_vals(Some(ball_data[0]), Some(ball_data[1]), Some(ball_data[2]));
        self.linear_velocity.set_vals(Some(ball_data[3]), Some(ball_data[4]), Some(ball_data[5]));
        self.angular_velocity.set_vals(Some(ball_data[6]), Some(ball_data[7]), Some(ball_data[8]));
    }

    pub fn forward(&mut self) -> Vec<f64> {
        let arr = &self.rotation_mtx();
        let partial_arr = arr.column(0);
        // [:, 0]
        // let ret_arr = partial_arr.to_owned();
        return partial_arr.to_owned().to_vec()
    }

    pub fn right(&mut self) -> Vec<f64> {
        let arr = self.rotation_mtx();
        let partial_arr = arr.column(1);
        return partial_arr.to_owned().to_vec()
    }

    pub fn left(&mut self) -> Vec<f64> {
        let arr = self.rotation_mtx();
        let partial_arr = arr.column(1);
        let res_arr = partial_arr.to_owned() * -1.;
        return res_arr.to_vec()
    }

    pub fn up(&mut self) -> Vec<f64> {
        let arr = self.rotation_mtx();
        let partial_arr = arr.column(2);
        return partial_arr.to_owned().to_vec()
    }

    pub fn pitch(&mut self) -> f64 {
        self.euler_angles().x
    }

    pub fn yaw(&mut self) -> f64 {
        self.euler_angles().y
    }

    pub fn roll(&mut self) -> f64 {
        self.euler_angles().z
    }

    pub fn euler_angles(&mut self) -> EulerAngle {
        if !self.has_computed_euler_angles {
            self.euler_angles = self.quaternion.quat_to_euler();
            self.has_computed_euler_angles = true;
        }
        return self.euler_angles
    }
    
    pub fn rotation_mtx(&mut self) -> Array2<f64> {
        if !self.has_computed_rot_mtx {
            self.rotation_mtx = self.quaternion.quat_to_rot_mtx();
            self.has_computed_rot_mtx = true;
        }
        return self.rotation_mtx.clone()
    }

    pub fn serialize(&self) -> Vec<f64> {
        let mut repr = Vec::<f64>::new();

        repr.extend(self.position.iter());
        repr.extend(self.quaternion.iter());
        repr.extend(self.linear_velocity.iter());
        repr.extend(self.angular_velocity.iter());
        repr.extend(self.euler_angles.iter());
        
        let mut row_vec = Vec::<f64>::new();
        for i in self.rotation_mtx.clone() {
            row_vec.push(i)
        }
        repr.append(&mut row_vec);

        return repr
    }
}
