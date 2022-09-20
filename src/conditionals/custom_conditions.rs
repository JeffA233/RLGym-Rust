use crate::gamestates::game_state::GameState;

use super::common_conditions::{TimeoutCondition, GoalScoredCondition, NoTouchTimeoutCondition};



struct CombinedTerminalConditions {
    timeout_condition: TimeoutCondition,
    no_touch_timeout_condition: NoTouchTimeoutCondition,
    goal_scored_condition: GoalScoredCondition,
    no_touch_kickoff_condition: NoTouchKickoffTimeoutCondition
}

impl CombinedTerminalConditions {
    fn new(tick_skip: usize) -> Self {
        CombinedTerminalConditions {
            timeout_condition: TimeoutCondition::new(400*120/tick_skip as i64),
            no_touch_timeout_condition: NoTouchTimeoutCondition::new(100*120/tick_skip as i64),
            goal_scored_condition: GoalScoredCondition::new(),
            no_touch_kickoff_condition: NoTouchKickoffTimeoutCondition::new(20*120/tick_skip as i64)
        }
    }

    fn reset(&mut self, _initial_state: &GameState) {
        self.timeout_condition.reset(_initial_state);
        self.no_touch_timeout_condition.reset(_initial_state);
        self.goal_scored_condition.reset(_initial_state);
        self.no_touch_kickoff_condition.reset(_initial_state);
    }

    fn is_terminal(&mut self, current_state: &GameState) -> bool {
        return [self.timeout_condition.is_terminal(current_state), 
        self.no_touch_timeout_condition.is_terminal(current_state), 
        self.goal_scored_condition.is_terminal(current_state), 
        self.no_touch_kickoff_condition.is_terminal(current_state)].into_iter().any(|x| x)
    }
}

pub struct NoTouchKickoffTimeoutCondition {
    steps: i64,
    max_steps: i64
}

impl NoTouchKickoffTimeoutCondition {
    pub fn new(max_steps: i64) -> Self {
        NoTouchKickoffTimeoutCondition {
            steps: 0,
            max_steps: max_steps
        }
    }

    pub fn reset(&mut self, _initial_state: &GameState) {
        self.steps = 0
    }

    pub fn is_terminal(&mut self, current_state: &GameState) -> bool {
        if current_state.ball.position[0] == 0. && current_state.ball.position[1] == 0. {
            if current_state.players.clone().into_iter().any(|x| x.ball_touched) {
                self.steps = 0;
                return false
            } else {
                self.steps += 1;
                return if self.steps >= self.max_steps {true} else {false}
            }
        } else {
            return false
        }
    }
}