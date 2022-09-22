
pub trait StateSetter {
    fn build_wrapper(&mut self, max_team_size: i32, spawn_opponents: bool) -> StateWrapper;
    fn reset(&mut self, state_wrapper: StateWrapper);
}