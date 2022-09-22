use crate::gamestates::{player_data::PlayerData};


pub struct CarWrapper {
    rotation: Vec<f32>,
    team_num: i32,
    id: i32,
    pub boost: f32,
    position: Vec<f32>,
    linear_velocity: Vec<f32>,
    angular_velocity: Vec<f32>
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
                rotation: vec![0.; 3],
                team_num: team_num,
                id: id,
                boost: 0.,
                position: vec![0.; 3],
                linear_velocity: vec![0.; 3],
                angular_velocity: vec![0.; 3]
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
            position: player_data.car_data.position.clone(),
            linear_velocity: player_data.car_data.linear_velocity.clone(),
            angular_velocity: player_data.car_data.angular_velocity.clone()
        }
    }

    pub fn set_rot(&mut self, pitch: Option<f32>, yaw: Option<f32>, roll: Option<f32>) {
        match pitch {
            Some(pitch) => self.rotation[0] = pitch,
            None => ()
        };
        match yaw {
            Some(yaw) => self.rotation[1] = yaw,
            None => ()
        };
        match roll {
            Some(roll) => self.rotation[2] = roll,
            None => ()
        };
    }

    pub fn set_pos(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        match x {
            Some(x) => self.position[0] = x,
            None => ()
        };
        match y {
            Some(y) => self.position[1] = y,
            None => ()
        };
        match z {
            Some(z) => self.position[2] = z,
            None => ()
        };
    }

    pub fn set_lin_vel(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        match x {
            Some(x) => self.linear_velocity[0] = x,
            None => ()
        };
        match y {
            Some(y) => self.linear_velocity[1] = y,
            None => ()
        };
        match z {
            Some(z) => self.linear_velocity[2] = z,
            None => ()
        };
    }

    pub fn set_ang_vel(&mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) {
        match x {
            Some(x) => self.angular_velocity[0] = x,
            None => ()
        };
        match y {
            Some(y) => self.angular_velocity[1] = y,
            None => ()
        };
        match z {
            Some(z) => self.angular_velocity[2] = z,
            None => ()
        };
    }

    pub fn encode(&self) -> Vec<f32> {
        let mut vec = Vec::<f32>::new();

        vec.push(self.id.clone() as f32);
        vec.append(&mut self.position.clone());
        vec.append(&mut self.linear_velocity.clone());
        vec.append(&mut self.angular_velocity.clone());
        vec.append(&mut self.rotation.clone());
        vec.push(self.boost.clone());
        let id = self.id;
        let boost = self.boost;


        // let vec_str: Vec<String>;

        // vec_str = vec.iter().map(|x| x.to_string()).collect();
        // let str = vec_str.join(" ");
        // format!("{id} {str} {boost}")
        return vec
    }
}