use std::f64::consts::PI;
use std::ops;

// use ndarray::*;


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

    pub fn into_array(&self) -> [f64; 3] {
        return [self.x, self.y, self.z]
    }

    pub fn to_vec(&self) -> Vec<f64> {
        return vec![self.x, self.y, self.z]
    }

    pub fn multiply_by_vel(&self, other_vel: &Velocity) -> Position {
        let x = self.x * other_vel.x;
        let y = self.y * other_vel.y;
        let z = self.z * other_vel.z;
        return Position { x, y, z }
    }

    pub fn divide_by_var(&self, var: f64) -> Position {
        let x = self.x / var;
        let y = self.y / var;
        let z = self.z / var;
        return Position { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        let mut running_val = 0.;
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        return running_val.sqrt()
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, other_pos: Position) -> Self::Output {
        return Position { 
            x: self.x + other_pos.x,
            y: self.y + other_pos.y,
            z: self.z + other_pos.z
        }
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        return Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul<Position> for Position {
    type Output = Position;

    fn mul(self, rhs: Position) -> Self::Output {
        return Position {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::Div<Position> for Position {
    type Output = Position;

    fn div(self, rhs: Position) -> Self::Output {
        return Position {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
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

    pub fn into_array(&self) -> [f64; 3] {
        return [self.x, self.y, self.z]
    }

    pub fn to_vec(&self) -> Vec<f64> {
        return vec![self.x, self.y, self.z]
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
        let mut running_val = 0.;
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        return running_val.sqrt()
    }

    pub fn scalar_projection(&self, dest_vec: &Position) -> f64 {
        let norm = dest_vec.norm();
        if norm == 0. {
            return 0.;
        }
        return (self.multiply_by_pos(dest_vec).into_array().iter().sum::<f64>())/norm
    }
}

impl ops::Add<Velocity> for Velocity {
    type Output = Velocity;

    fn add(self, other_pos: Velocity) -> Self::Output {
        return Velocity { 
            x: self.x + other_pos.x,
            y: self.y + other_pos.y,
            z: self.z + other_pos.z
        }
    }
}

impl ops::Sub<Velocity> for Velocity {
    type Output = Velocity;

    fn sub(self, rhs: Velocity) -> Self::Output {
        return Velocity {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul<Velocity> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: Velocity) -> Self::Output {
        return Velocity {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl ops::Div<Velocity> for Velocity {
    type Output = Velocity;

    fn div(self, rhs: Velocity) -> Self::Output {
        return Velocity {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
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
        let mut running_val = 0.;
        running_val += self.w.powi(2);
        running_val += self.x.powi(2);
        running_val += self.y.powi(2);
        running_val += self.z.powi(2);
        return running_val.sqrt()
    }

    pub fn dot(&self, quat: Quaternion) -> f64 {
        self.w * quat.w + 
        self.x * quat.x +
        self.y * quat.y +
        self.z * quat.z 
    }

    /// quat Vec to rotation matrix Array2
    pub fn quat_to_rot_mtx(&self) -> RotationMatrix {
        let mut theta = RotationMatrix::zeros();

        let norm = self.dot(*self);

        let w = -&self.w;
        let x = -&self.x;
        let y = -&self.y;
        let z = -&self.z;

        // let s: f64 = 1.0 / norm;

        if norm != 0. {
            let s: f64 = 1.0 / norm;

            // front direction
            theta.array[0][0] = 1. - 2. * s * (y * y + z * z);
            theta.array[1][0] = 2. * s * (x * y + z * w);
            theta.array[2][0] = 2. * s * (x * z - y * w);

            // left direction
            theta.array[0][1] = 2. * s * (x * y - z * w);
            theta.array[1][1] = 1. - 2. * s * (x * x + z * z);
            theta.array[2][1] = 2. * s * (y * z + x * w);

            // up direction
            theta.array[0][2] = 2. * s * (x * z + y * w);
            theta.array[1][2] = 2. * s * (y * z - x * w);
            theta.array[2][2] = 1. - 2. * s * (x * x + y * y);
        }

        return theta;
    }

    pub fn quat_to_euler(&self) -> EulerAngle {
        let w: f64 = self.w;
        let x: f64 = self.x;
        let y: f64 = self.y;
        let z: f64 = self.z;
    
        let sinr_cosp: f64 = 2. * (w * x + y * z);
        let cosr_cosp: f64 = 1. - 2. * (x * x + y * y);
        let sinp: f64 = 2. * (w * y - z * x);
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

        EulerAngle { pitch: -pitch, yaw: yaw, roll: -roll }
    }

    pub fn into_array(&self) -> [f64; 4] {
        return [self.w, self.x, self.y, self.z]
    }
}

#[derive(Clone, Copy, Default)]
pub struct EulerAngle {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64
}

impl EulerAngle {
    pub fn set_vals(&mut self, pitch: Option<f64>, yaw: Option<f64>, roll: Option<f64>) {
        match pitch {
            Some(val) => self.pitch = val,
            None => ()
        }
        match yaw {
            Some(val) => self.yaw = val,
            None => ()
        }
        match roll {
            Some(val) => self.roll = val,
            None => ()
        }
    }

    pub fn into_array(&self) -> [f64; 3] {
        return [self.pitch, self.yaw, self.roll]
    }
}

#[derive(Default, Clone, Copy)]
pub struct RotationMatrix {
    pub array: [[f64; 3]; 3]
}

impl RotationMatrix {
    // pub fn get_val(&self, row: usize, col: usize) -> f64 {
    //     self.array[row][col]
    // }

    pub fn column(&self, col: usize) -> [f64; 3] {
        let val1 = self.array[0][col];
        let val2 = self.array[1][col];
        let val3 = self.array[2][col];
        return [val1, val2, val3];
    }

    pub fn row(&self, row: usize) -> [f64; 3] {
        let val1 = self.array[row][0];
        let val2 = self.array[row][1];
        let val3 = self.array[row][2];
        return [val1, val2, val3];
    }

    pub fn zeros() -> RotationMatrix {
        RotationMatrix {
            array: [[0.; 3]; 3]
        }
    }

    pub fn into_array(&self) -> [[f64; 3]; 3] {
        return self.array
    }

    pub fn into_flat_array(&self) -> [f64; 9] {
        let mut row_vec = [0.; 9];
        let mut i = 0;
        // for col in self.array {
        //     for row_val in col {
        //         row_vec[i] = row_val;
        //         i += 1;
        //     }
        // }
        for idx in 0..3 {
            for col in self.array {
                row_vec[i] = col[idx];
                i += 1;
            }
        }
        return row_vec
    }
}

// end of helper structs
// -------------------------------------------------------------------------------------------
// start of PhysicsObject struct

/// Struct that holds any kind of physics data for car/ball
#[derive(Default, Clone, Copy)]
pub struct PhysicsObject {
    pub position: Position,
    pub quaternion: Quaternion,
    pub linear_velocity: Velocity,
    pub angular_velocity: Velocity,
    pub euler_angles: EulerAngle,
    pub rotation_mtx: RotationMatrix,
    pub has_computed_rot_mtx: bool,
    pub has_computed_euler_angles: bool
}

impl PhysicsObject {
    pub fn new() -> Self {
        PhysicsObject {
            position: Position::default(),
            quaternion: Quaternion::default(),
            linear_velocity: Velocity::default(),
            angular_velocity: Velocity::default(),
            euler_angles: EulerAngle::default(),
            rotation_mtx: RotationMatrix::zeros(),
            has_computed_euler_angles: false,
            has_computed_rot_mtx: false
        }
    }

    pub fn decode_car_data(&mut self, car_data: &[f64]) {
        self.position.set_vals(Some(car_data[0]), Some(car_data[1]), Some(car_data[2]));
        self.quaternion.set_vals(Some(car_data[3]), Some(car_data[4]), Some(car_data[5]), Some(car_data[6]));
        self.linear_velocity.set_vals(Some(car_data[7]), Some(car_data[8]), Some(car_data[9]));
        self.angular_velocity.set_vals(Some(car_data[10]), Some(car_data[11]), Some(car_data[12]));
    }

    pub fn decode_ball_data(&mut self, ball_data: &[f64]) {
        self.position.set_vals(Some(ball_data[0]), Some(ball_data[1]), Some(ball_data[2]));
        self.linear_velocity.set_vals(Some(ball_data[3]), Some(ball_data[4]), Some(ball_data[5]));
        self.angular_velocity.set_vals(Some(ball_data[6]), Some(ball_data[7]), Some(ball_data[8]));
    }

    pub fn forward(&mut self) -> [f64; 3] {
        let arr = &self.rotation_mtx();
        let partial_arr = arr.column(0);
        return partial_arr
    }

    pub fn right(&mut self) -> [f64; 3] {
        let arr = self.rotation_mtx();
        let partial_arr = arr.column(1);
        return partial_arr
    }

    pub fn left(&mut self) -> [f64; 3] {
        let arr = self.rotation_mtx();
        let mut partial_arr = arr.column(1);
        for val in partial_arr.iter_mut() {
            *val = *val*-1.;
        }
        return partial_arr
    }

    pub fn up(&mut self) -> [f64; 3] {
        let arr = self.rotation_mtx();
        let partial_arr = arr.column(2);
        return partial_arr
    }

    pub fn pitch(&mut self) -> f64 {
        self.euler_angles().pitch
    }

    pub fn yaw(&mut self) -> f64 {
        self.euler_angles().yaw
    }

    pub fn roll(&mut self) -> f64 {
        self.euler_angles().roll
    }

    pub fn euler_angles(&mut self) -> EulerAngle {
        if !self.has_computed_euler_angles {
            self.euler_angles = self.quaternion.quat_to_euler();
            self.has_computed_euler_angles = true;
        }
        return self.euler_angles
    }
    
    pub fn rotation_mtx(&mut self) -> RotationMatrix {
        if !self.has_computed_rot_mtx {
            self.rotation_mtx = self.quaternion.quat_to_rot_mtx();
            self.has_computed_rot_mtx = true;
        }
        return self.rotation_mtx
    }

    pub fn serialize(&mut self) -> Vec<f64> {
        let mut repr = Vec::<f64>::with_capacity(25);

        repr.extend(self.position.into_array().iter());
        repr.extend(self.quaternion.into_array().iter());
        repr.extend(self.linear_velocity.into_array().iter());
        repr.extend(self.angular_velocity.into_array().iter());
        repr.extend(self.euler_angles.into_array().iter());
        
        // let mut row_vec = Vec::<f64>::with_capacity(9);
        let row_vec = self.rotation_mtx().into_flat_array();
        repr.extend(row_vec.iter());

        return repr
    }
}
