use crate::gamestates::physics_object::PhysicsObject;



pub struct PhysicsWrapper {
    position: Vec<f64>,
    linear_velocity: Vec<f64>,
    angular_velocity: Vec<f64>
}

impl PhysicsWrapper {
    pub fn new(phys_obj: Option<&PhysicsObject>) -> Self {
        let wrapper = match phys_obj {
            Some(phys_obj) => PhysicsWrapper::_read_from_physics_object(phys_obj),
            None => PhysicsWrapper {
                position: vec![0., 0., 91.25],
                linear_velocity: vec![0.; 3],
                angular_velocity: vec![0.; 3]
            }
        };
        return wrapper
    }

    fn _read_from_physics_object(phys_obj: &PhysicsObject) -> PhysicsWrapper {
        PhysicsWrapper {
            position: phys_obj.position.clone(),
            linear_velocity: phys_obj.linear_velocity.clone(),
            angular_velocity: phys_obj.angular_velocity.clone()
        }
    }

    pub fn set_pos(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
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

    pub fn set_lin_vel(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
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

    pub fn set_ang_vel(&mut self, x: Option<f64>, y: Option<f64>, z: Option<f64>) {
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

    pub fn encode(&self) -> Vec<f64> {
        let mut vec = Vec::<f64>::new();

        vec.append(&mut self.position.clone());
        vec.append(&mut self.linear_velocity.clone());
        vec.append(&mut self.angular_velocity.clone());

        // let vec_str: Vec<String>;

        // vec_str = vec.iter().map(|x| x.to_string()).collect();
        // vec_str.join(" ")
        return vec
    }
}