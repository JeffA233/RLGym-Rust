use crate::gamestates::physics_object::PhysicsObject;



pub struct PhysicsWrapper {
    position: Vec<f32>,
    linear_velocity: Vec<f32>,
    angular_velocity: Vec<f32>
}

impl PhysicsWrapper {
    pub fn new(phys_obj: Option<&PhysicsObject>) -> Self {
        let wrapper = match phys_obj {
            Some(phys_obj) => PhysicsWrapper::_read_from_physics_object(phys_obj),
            None => PhysicsWrapper {
                position: vec![0.; 3],
                linear_velocity: vec![0.; 3],
                angular_velocity: vec![0.; 3]
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

    pub fn encode(&self) -> String {
        let mut vec = Vec::<f32>::new();

        vec.append(&mut self.position.clone());
        vec.append(&mut self.linear_velocity.clone());
        vec.append(&mut self.angular_velocity.clone());

        let mut vec_str = Vec::<String>::new();

        vec_str = vec.iter().map(|x| x.to_string()).collect();
        vec_str.join(" ")
    }
}