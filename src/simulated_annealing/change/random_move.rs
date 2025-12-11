use crate::simulated_annealing::{
    change::{Change, random_helpers::sample_centered_int},
    state::State,
};
use rand::Rng;

pub struct RandomMoveChange {
    action_index: usize,
    old_time: u32,
    new_time: u32,
}
impl Change for RandomMoveChange {
    fn apply(&self, state: &mut State) {
        let constant_actions = state.get_constant_actions_mut();
        let action = &mut constant_actions[self.action_index];
        *action.get_start_time_mut() = self.new_time as i32;
    }
    fn undo(&self, state: &mut State) {
        let constant_actions = state.get_constant_actions_mut();
        let action = &mut constant_actions[self.action_index];
        *action.get_start_time_mut() = self.old_time as i32;
    }
}

impl RandomMoveChange {
    pub fn new_random<R: Rng>(rng: &mut R, state: &State, sigma: f64) -> Self {
        let constant_actions = state.get_constant_actions();
        let action_index = rng.random_range(0..constant_actions.len());
        let action = &constant_actions[action_index];
        let action_ref = action.get_action();
        let old_time = action.get_start_time() as u32;
        let start_bound = action_ref.get_start_from() as u32;
        let end_bound = (action_ref.get_end_before() - action_ref.duration) as u32;
        let mut new_time = old_time;
        while new_time == old_time {
            new_time = sample_centered_int(start_bound, end_bound, old_time, sigma, rng);
        }
        Self {
            action_index,
            old_time,
            new_time,
        }
    }
}
