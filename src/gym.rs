use crate::action_parsers;
use crate::common_values;
use crate::communication;
use crate::communication::communication_handler::CommunicationHandler;
use crate::communication::communication_handler::format_pipe_id;
use crate::communication::message::RLGYM_CONFIG_MESSAGE_HEADER;
use crate::conditionals;
use crate::envs;
use crate::gamelaunch;
use crate::gamelaunch::launch::LaunchPreference;
use crate::gamelaunch::launch::launch_rocket_league;
use crate::gamestates;
use crate::gamestates::game_state::GameState;
use crate::math;
use crate::obs_builders;
use crate::reward_functions;
use crate::state_setters;
use crate::envs::game_match::game_match;
use ndarray::*;
use subprocess::Popen;
use subprocess::Result;
use std::thread::JoinHandle;
use std::thread::Thread;
use std::thread;

pub struct Gym {
    pub _match: game_match,
    pub observation_space: ArrayD<usize>,
    pub action_space: ArrayD<usize>,
    pub _launch_preference: String,
    pub _use_injector: bool,
    pub _force_paging: bool,
    pub _raise_on_crash: bool,
    pub _comm_handler: CommunicationHandler,
    pub _local_pipe_name: String,
    pub _local_pipe_id: usize,
    pub _game_process: Popen,
    pub _minimizing_thread: JoinHandle<()>,
    pub _minimized: bool,
    pub _auto_minimize: bool,
    pub _prev_state: GameState
}

impl Gym {
    pub fn new(game_match: game_match, pipe_id: Option<usize>, launch_preference: Option<String>, use_injector: Option<bool>, force_paging: Option<bool>, raise_on_crash: Option<bool>, auto_minimize: Option<bool>) -> Self {
        let pipe_id = match pipe_id {
            Some(pipe_id) => pipe_id,
            None => 0
        };
        let launch_preference = match launch_preference {
            Some(preference) => preference,
            None => LaunchPreference::new().EPIC
        };
        let use_injector = match use_injector {
            Some(use_injector) => use_injector,
            None => true
        };
        let force_paging = match force_paging {
            Some(force_paging) => force_paging,
            None => false
        };
        let raise_on_crash = match raise_on_crash {
            Some(raise) => raise,
            None => false
        };
        let auto_minimize = match auto_minimize {
            Some(auto_minimize) => auto_minimize,
            None => false
        };

        let mut comm_handler = CommunicationHandler::new();
        let pipe_name = &format_pipe_id(pipe_id);

        let proc = launch_rocket_league(pipe_name.to_string(), &launch_preference);
        comm_handler.open_pipe(Some(&pipe_name), None);
        comm_handler.send_message(None, Some(RLGYM_CONFIG_MESSAGE_HEADER.to_vec()), Some(game_match.get_config()));
        // TODO thread that minimizes the game
        let handle = thread::spawn(||println!("hello"));
        // if force_paging {
        //     page_rocket_league()
        // }
        let observation_space = game_match.observation_space.clone();
        let action_space = game_match.action_space.clone(); 
        Gym {
            _match: game_match,
            observation_space: observation_space,
            action_space: action_space,
            _launch_preference: launch_preference,
            _use_injector: use_injector,
            _force_paging: force_paging,
            _raise_on_crash: raise_on_crash,
            _comm_handler: comm_handler,
            _local_pipe_name: pipe_name.to_string(),
            _local_pipe_id: pipe_id,
            _game_process: proc,
            _minimizing_thread: handle,
            _minimized: false,
            _auto_minimize: auto_minimize,
            _prev_state: GameState::new(),
        }
    }
}

