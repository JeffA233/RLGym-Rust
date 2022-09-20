use std::thread::current;

use crate::gamestates::game_state::GameState;



pub struct TimeoutCondition {
    steps: i64,
    max_steps: i64
}

impl TimeoutCondition {
    pub fn new(max_steps: i64) -> Self {
        TimeoutCondition {
            steps: 0,
            max_steps: max_steps
        }
    }

    pub fn reset(&mut self, _initial_state: &GameState) {
        self.steps = 0;
    }

    pub fn is_terminal(&mut self, _current_state: &GameState) -> bool {
        self.steps += 1;
        return if self.steps >= self.max_steps {true} else {false}
    }
}

pub struct NoTouchTimeoutCondition {
    steps: i64,
    max_steps: i64
}

impl NoTouchTimeoutCondition {
    pub fn new(max_steps: i64) -> Self {
        NoTouchTimeoutCondition {
            steps: 0,
            max_steps: max_steps
        }
    }

    pub fn reset(&mut self, _initial_state: &GameState) {
        self.steps = 0
    }

    pub fn is_terminal(&mut self, current_state: &GameState) -> bool {
        if current_state.players.clone().into_iter().any(|x| x.ball_touched) {
            self.steps = 0;
            return false
        } else {
            self.steps += 1;
            return if self.steps >= self.max_steps {true} else {false}
        }
    }
}

pub struct GoalScoredCondition {
    blue_score: i64,
    orange_score: i64
}

impl GoalScoredCondition {
    pub fn new() -> Self {
        GoalScoredCondition {
            blue_score: 0,
            orange_score: 0
        }
    }

    pub fn reset(&mut self, _initial_state: &GameState) {

    }

    pub fn is_terminal(&mut self, current_state: &GameState) -> bool {
        if current_state.blue_score != self.blue_score || current_state.orange_score != self.orange_score {
            self.blue_score = current_state.blue_score;
            self.orange_score = current_state.orange_score;
            return true
        } else {
            return false
        }
    }
}
