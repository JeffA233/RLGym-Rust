use crate::communication::communication_handler::{CommunicationHandler, format_pipe_id};
use crate::communication::message::{RLGYM_CONFIG_MESSAGE_HEADER, RLGYM_RESET_GAME_STATE_MESSAGE_HEADER, RLGYM_STATE_MESSAGE_HEADER, RLGYM_AGENT_ACTION_IMMEDIATE_RESPONSE_MESSAGE_HEADER};

use crate::gamelaunch::launch::run_injector;
use crate::gamelaunch::launch::{LaunchPreference, launch_rocket_league};
use crate::gamelaunch::minimize::toggle_rl_windows;

use crate::gamestates::game_state::GameState;

use crate::envs::game_match::GameMatch;

use subprocess::Popen;

use std::collections::HashMap;

use std::thread;
use std::time::Duration;

/// Base Gym struct for RLGym-Rust
pub struct Gym {
    pub _game_match: GameMatch,
    pub observation_space: Vec<usize>,
    pub action_space: Vec<usize>,
    pub _launch_preference: String,
    pub _use_injector: bool,
    pub _force_paging: bool,
    pub _raise_on_crash: bool,
    pub _comm_handler: CommunicationHandler,
    pub _local_pipe_name: String,
    pub _local_pipe_id: usize,
    pub _game_process: Popen,
    // pub _minimizing_thread: JoinHandle<()>,
    pub _minimized: bool,
    pub _auto_minimize: bool,
    pub _prev_state: GameState
}

impl Gym {
    /// Creates a new instance of a gym and launches + connects to Rocket League instance
    pub fn new(game_match: GameMatch, pipe_id: Option<usize>, launch_preference: Option<String>, use_injector: Option<bool>, force_paging: Option<bool>, raise_on_crash: Option<bool>, auto_minimize: Option<bool>) -> Self {
        let pipe_id = match pipe_id {
            Some(pipe_id) => pipe_id,
            None => 0
        };
        let launch_preference = match launch_preference {
            Some(preference) => preference,
            None => LaunchPreference::new().epic
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
        if use_injector {
            thread::sleep(Duration::new(3, 0));
            run_injector();
        }
        // let proc_pid = proc.pid().unwrap();
        comm_handler.open_pipe(Some(&pipe_name), None);
        comm_handler.send_message(None, Some(RLGYM_CONFIG_MESSAGE_HEADER.to_vec()), Some(game_match.get_config().to_vec()));
        // if force_paging {
        //     page_rocket_league()
        // }
        let observation_space = game_match.observation_space.clone();
        let action_space = game_match.action_space.clone(); 
        let mut gym = Gym {
            _game_match: game_match,
            observation_space,
            action_space,
            _launch_preference: launch_preference,
            _use_injector: use_injector,
            _force_paging: force_paging,
            _raise_on_crash: raise_on_crash,
            _comm_handler: comm_handler,
            _local_pipe_name: pipe_name.to_string(),
            _local_pipe_id: pipe_id,
            _game_process: proc,
            // _minimizing_thread: handle,
            _minimized: false,
            _auto_minimize: auto_minimize,
            _prev_state: GameState::new(None),
        };
        gym.reset(None, None);
        if gym._auto_minimize {
            gym._minimize_game();
        }
        return gym
    }

    fn _minimize_game(&mut self) {
        thread::spawn(|| toggle_rl_windows(true));
        // toggle_rl_windows(true);
        self._minimized = true;
    }

    pub fn reset(&mut self, _return_info: Option<bool>, seed: Option<u64>) -> Vec<Vec<f64>> {
        // let _return_info = match _return_info {
        //     Some(return_info) => return_info,
        //     None => false
        // };
        match seed {
            Some(seed) => self._game_match.set_seeds(seed),
            None => ()
        }

        let state_str = self._game_match.get_reset_state();

        self._comm_handler.send_message(None, Some(RLGYM_RESET_GAME_STATE_MESSAGE_HEADER.to_vec()), Some(state_str));

        let mut state = self._receive_state();
        self._game_match.episode_reset(&state);
        self._prev_state = state.clone();

        let obs = self._game_match.build_observations(&mut state);
        // TODO return Option except that state and get_result don't match
        // if _return_info {
        //     let mut h_m = HashMap::<&str,f64>::new();
        //     h_m.insert("result", self._game_match.get_result(state) as f64);
        // }
        return obs
    }

    pub fn step(&mut self, actions: Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<f64>, bool, HashMap<String, f64>) {
        let actions = self._game_match.parse_actions(actions, &self._prev_state);
        let act_res = self._send_actions(actions);

        if !act_res {
            self.close();
            panic!("closed gym because of action error")
        }

        let state = self._receive_state();

        let obs = self._game_match.build_observations(&state);
        let done = self._game_match.is_done(&state);
        self._prev_state = state.clone();
        let reward = self._game_match.get_rewards(&state, done);
        let mut info = HashMap::<String,f64>::new();
        info.insert("result".to_string(), self._game_match.get_result(&state) as f64);
        return (obs, reward, done, info)
    }

    pub fn close(&mut self) {
        self._game_process.terminate().unwrap();
        self._comm_handler.close_pipe();
    }

    fn _receive_state(&mut self) -> GameState {
        let message = self._comm_handler.receive_message(Some(RLGYM_STATE_MESSAGE_HEADER.to_vec()));
        if message.body[0] == -999999. {
            self.close();
            panic!("closed gym because of comm error");
        }
        return self._game_match.parse_state(message.body)
    }

    fn _send_actions(&mut self, actions: Vec<Vec<f64>>) -> bool {
        let mut result = true;
        for action in &actions {
            // assert!(action.len() == 8, "action was not of length 8");
            if action.len() != 8 {
                let act_str = format!("{:?}", action);
                println!("action was not of length 8, action was: {act_str}");
                result = false;
            }
        }

        let actions_formatted = self._game_match.format_actions(actions);

        self._comm_handler.send_message(None, Some(RLGYM_AGENT_ACTION_IMMEDIATE_RESPONSE_MESSAGE_HEADER.to_vec()), Some(actions_formatted));

        return result
    }
}

