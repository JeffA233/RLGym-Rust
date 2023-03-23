use crate::{conditionals::terminal_condition::TerminalCondition, reward_functions::default_reward::RewardFn, obs_builders::obs_builder::ObsBuilder, action_parsers::action_parser::ActionParser, state_setters::state_setter::StateSetter, envs::game_match::GameMatch, gym::Gym, gamelaunch::launch::LaunchPreference};
use std::process::id;

/// General generator function for the gym
pub fn make(
    game_speed: Option<f64>,
    tick_skip: Option<usize>,
    spawn_opponents: Option<bool>,
    team_size: Option<usize>,
    gravity: Option<f64>,
    boost_consumption: Option<f64>,
    pipe_name: Option<usize>,
    terminal_condition: Box<dyn TerminalCondition + Send>,
    reward_fn: Box<dyn RewardFn + Send>,
    obs_builder: Vec<Box<dyn ObsBuilder + Send>>,
    action_parser: Box<dyn ActionParser + Send>,
    state_setter: Box<dyn StateSetter + Send>,
    launch_preference: Option<String>,
    use_injector: bool,
    force_paging: bool,
    raise_on_crash: bool,
    auto_minimize: bool
) -> Gym {
    let game_speed = match game_speed {
        Some(game_speed) => game_speed,
        None => 100.
    };
    let tick_skip = match tick_skip {
        Some(tick_skip) => tick_skip,
        None => 8
    };
    let spawn_opponents = match spawn_opponents {
        Some(spawn_opponents) => spawn_opponents,
        None => true
    };
    let team_size = match team_size {
        Some(team_size) => team_size,
        None => 1
    };
    let gravity = match gravity {
        Some(gravity) => gravity,
        None => 1.
    };
    let boost_consumption = match boost_consumption {
        Some(boost_consumption) => boost_consumption,
        None => 1.
    };
    let launch_pref = match launch_preference {
        Some(launch_pref) => launch_pref,
        None => LaunchPreference::new().epic
    };
    let game_match = GameMatch::new(
        reward_fn, 
        terminal_condition,
        obs_builder,
        action_parser,
        state_setter,
Some(team_size),
Some(tick_skip),
Some(game_speed),
Some(gravity),
Some(boost_consumption),
Some(spawn_opponents));
    
    Gym::new(game_match, 
        Some(match pipe_name {
            Some(pipe_name) => pipe_name, 
            None => id() as usize}), 
        Some(launch_pref), 
        Some(use_injector), 
        Some(force_paging),
    Some(raise_on_crash),
Some(auto_minimize))
}