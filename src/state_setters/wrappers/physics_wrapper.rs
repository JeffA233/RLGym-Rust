use crate::gamestates::physics_object::{PhysicsObject, Position, Velocity};


/// Physics wrapper that allows for easy modification of a PhysicsObject
pub struct PhysicsWrapper {
    position: Position,
    linear_velocity: Velocity,
    angular_velocity: Velocity
}

impl PhysicsWrapper {
    pub fn new(phys_obj: Option<&PhysicsObject>) -> Self {
        let wrapper = match phys_obj {
            Some(phys_obj) => PhysicsWrapper::_read_from_physics_object(phys_obj),
            None => PhysicsWrapper {
                // position: vec![0., 0., 91.25],
                position: Position { x: 0., y: 0., z: 91.25 },
                linear_velocity: Velocity { x: 0., y: 0., z: 0. },
                angular_velocity: Velocity { x: 0., y: 0., z: 0. }
            }
        };
        return wrapper
    }

    fn _read_from_physics_object(phys_obj: &PhysicsObject) -> PhysicsWrapper {
        PhysicsWrapper {
            position: phys_obj.position,
            linear_velocity: phys_obj.linear_velocity,
            angular_velocity: phys_obj.angular_velocity
        }
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

        vec.extend(self.position.into_array().iter());
        vec.extend(self.linear_velocity.into_array().iter());
        vec.extend(self.angular_velocity.into_array().iter());

        // let vec_str: Vec<String>;

        // vec_str = vec.iter().map(|x| x.to_string()).collect();
        // vec_str.join(" ")
        return vec
    }
}