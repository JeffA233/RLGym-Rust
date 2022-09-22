use super::wrappers::state_wrapper::StateWrapper;


pub trait StateSetter {
    fn build_wrapper(&mut self, max_team_size: i32, spawn_opponents: bool) -> StateWrapper  {
        StateWrapper::new(Some(max_team_size), if spawn_opponents {Some(max_team_size)} else {Some(0)}, None)
    }
    fn reset(&mut self, state_wrapper: &mut StateWrapper);
}