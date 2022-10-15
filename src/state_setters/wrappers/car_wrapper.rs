use crate::gamestates::{player_data::PlayerData, physics_object::{EulerAngle, Position, Velocity}};

/// Car wrapper that allows for easy modification of all of the units in a car (PlayerData, PhysicsObject)
pub struct CarWrapper {
    rotation: EulerAngle,
    pub team_num: i32,
    id: i32,
    pub boost: f64,
    position: Position,
    linear_velocity: Velocity,
    angular_velocity: Velocity
}

impl CarWrapper {
    pub fn new(team_num: Option<i32>, id: Option<i32>, player_data: Option<&mut PlayerData>) -> Self {
        let team_num = match team_num {
            Some(team_num) => team_num,
            None => -1
        };
        let id = match id {
            Some(id) => id,
            None => -1
        };
        let wrapper = match player_data {
            Some(player_data) => CarWrapper::_read_from_player_data(player_data),
            None => CarWrapper {
                rotation: EulerAngle { pitch: 0., yaw: 0., roll: 0. },
                team_num: team_num,
                id: id,
                boost: 0.,
                position: Position { x: 0., y: 0., z: 0. },
                linear_velocity: Velocity { x: 0., y: 0., z: 0. },
                angular_velocity: Velocity { x: 0., y: 0., z: 0. }
            }
        };
        return wrapper
    }

    fn _read_from_player_data(player_data: &mut PlayerData) -> CarWrapper {
        CarWrapper {
            rotation: player_data.car_data.euler_angles(),
            team_num: player_data.team_num,
            id: player_data.car_id,
            boost: player_data.boost_amount,
            position: player_data.car_data.position,
            linear_velocity: player_data.car_data.linear_velocity,
            angular_velocity: player_data.car_data.angular_velocity
        }
    }

    pub fn set_rot(&mut self, pitch: Option<f64>, yaw: Option<f64>, roll: Option<f64>) {
        match pitch {
            Some(pitch) => self.rotation.pitch = pitch,
            None => ()
        };
        match yaw {
            Some(yaw) => self.rotation.yaw = yaw,
            None => ()
        };
        match roll {
            Some(roll) => self.rotation.roll = roll,
            None => ()
        };
    }

    pub fn set_pos(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
        match x {
            Some(x) => self.position.x = x,
            None => ()
        };
        match y {
            Some(y) => self.position.y = y,
            None => ()
        };
        match z {
            Some(z) => self.position.z = z,
            None => ()
        };
    }

    pub fn set_lin_vel(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
        match x {
            Some(x) => self.linear_velocity.x = x,
            None => ()
        };
        match y {
            Some(y) => self.linear_velocity.y = y,
            None => ()
        };
        match z {
            Some(z) => self.linear_velocity.z = z,
            None => ()
        };
    }

    pub fn set_ang_vel(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
        match x {
            Some(x) => self.angular_velocity.x = x,
            None => ()
        };
        match y {
            Some(y) => self.angular_velocity.y = y,
            None => ()
        };
        match z {
            Some(z) => self.angular_velocity.z = z,
            None => ()
        };
    }

    pub fn encode(&self) -> Vec<f64> {
        let mut vec = Vec::<f64>::new();

        vec.push(self.id as f64);
        vec.extend(self.position.into_array().iter());
        vec.extend(self.linear_velocity.into_array().iter());
        vec.extend(self.angular_velocity.into_array().iter());
        vec.extend(self.rotation.into_array().iter());
        vec.push(self.boost);

        // let vec_str: Vec<String>;

        // vec_str = vec.iter().map(|x| x.to_string()).collect();
        // let str = vec_str.join(" ");
        // format!("{id} {str} {boost}")
        return vec
    }
}