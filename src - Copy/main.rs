mod action_parsers;
mod common_values;
mod communication;
mod conditionals;
mod envs;
mod gamelaunch;
pub mod gamestates;
mod math;
mod obs_builders;
mod reward_functions;
mod state_setters;

// math.norm_func();

fn main() {
    println!("Hello, world!");
    let gamest = gamestates::physics_object::PhysicsObject::default();
    let pos = gamest.position;
    let pos_str = pos.iter().map(|x| "{x}").collect::<String>();
    println!("{pos_str}")
}
