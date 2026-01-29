pub mod multi_change;
mod random_helpers;
mod random_move;
use crate::simulated_annealing::state::State;

pub trait Change {
    fn apply(&self, state: &mut State);
    fn undo(&self, state: &mut State);
}
