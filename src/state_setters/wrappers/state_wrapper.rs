use crate::gamestates::game_state::{GameState};

use super::{physics_wrapper::PhysicsWrapper, car_wrapper::CarWrapper};



const BLUE_ID1: i32 = 1;
const ORANGE_ID1: i32 = 5;

pub struct StateWrapper {
    pub ball: PhysicsWrapper,
    pub cars: Vec<CarWrapper>
}

impl StateWrapper {
    pub fn new(blue_count: Option<i32>, orange_count: Option<i32>, game_state: Option<&mut GameState>) -> Self {
        let blue_count = match blue_count {
            Some(blue_count) => blue_count,
            None => 0
        };
        let orange_count = match orange_count {
            Some(orange_count) => orange_count,
            None => 0
        };
        let wrapper = match game_state {
            Some(game_state) => StateWrapper::_read_from_gamestate(game_state),
            None => {
                let mut cars = Vec::<CarWrapper>::new();
                for i in 0..blue_count {
                    cars.push(CarWrapper::new(Some(0), Some(BLUE_ID1 + i), None))
                }
                for i in 0..orange_count {
                    cars.push(CarWrapper::new(Some(1), Some(ORANGE_ID1 + i), None))
                }
                StateWrapper {
                    ball: PhysicsWrapper::new(None),
                    cars: cars
                }
            }
        };
        return wrapper
    }

    fn _read_from_gamestate(game_state: &mut GameState) -> StateWrapper {
        let mut cars = Vec::<CarWrapper>::new();
        // let players = &mut game_state.players;
        for mut player in &mut game_state.players {
            cars.push(CarWrapper::new(None, None, Some(&mut player)))
        }
        StateWrapper {
            ball: PhysicsWrapper::new(Some(&game_state.ball)),
            cars: cars
        }
    }

    pub fn format_state(&self) -> String {
        let ball_str = self.ball.encode();
        let mut car_str_vec = Vec::<String>::new();
        for c in &self.cars {
            car_str_vec.push(c.encode());
        }
        let car_str = car_str_vec.join(" ");
        format!("{ball_str} {car_str}")
    }
}