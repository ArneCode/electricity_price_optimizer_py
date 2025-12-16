use crate::{
    simulated_annealing::{
        change::{Change, random_helpers::sample_centered_int},
        state::State,
    },
    time::Time,
};
use rand::Rng;

pub struct RandomMoveChange {
    action_index: usize,
    old_time: Time,
    new_time: Time,
}
impl Change for RandomMoveChange {
    fn apply(&self, state: &mut State) {
        let constant_actions = state.get_constant_actions_mut();
        let action = &mut constant_actions[self.action_index];
        *action.get_start_time_mut() = self.new_time;

        println!(
            "Moved action {} from {} to {}",
            self.action_index, self.old_time, self.new_time
        );
    }
    fn undo(&self, state: &mut State) {
        let constant_actions = state.get_constant_actions_mut();
        let action = &mut constant_actions[self.action_index];
        *action.get_start_time_mut() = self.old_time;
    }
}

impl RandomMoveChange {
    pub fn new_random<R: Rng>(rng: &mut R, state: &State, sigma: f64) -> Self {
        let constant_actions = state.get_constant_actions();
        let action_index = rng.random_range(0..constant_actions.len());
        let action = &constant_actions[action_index];
        let action_ref = action.get_action();
        let old_time = action.get_start_time().get_minutes() as u32;
        let start_bound = action_ref.get_start_from().get_minutes() as u32;
        let end_bound =
            (action_ref.get_end_before().get_minutes() - action_ref.duration.get_minutes()) as u32;
        let mut new_time = old_time;
        while new_time == old_time {
            new_time = sample_centered_int(start_bound, end_bound, old_time, sigma, rng);
        }
        Self {
            action_index,
            old_time: Time::new(0, old_time),
            new_time: Time::new(0, new_time),
        }
    }
}
