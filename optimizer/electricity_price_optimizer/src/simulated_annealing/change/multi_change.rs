use crate::simulated_annealing::{
    change::{Change, random_move::RandomMoveChange},
    state::State,
};

pub struct MultiChange {
    changes: Vec<Box<dyn Change>>,
}

impl Change for MultiChange {
    fn apply(&self, state: &mut State) {
        for change in &self.changes {
            change.apply(state);
        }
    }

    fn undo(&self, state: &mut State) {
        for change in self.changes.iter().rev() {
            change.undo(state);
        }
    }
}

impl MultiChange {
    pub fn new_random<R: rand::Rng>(
        rng: &mut R,
        state: &State,
        random_move_sigma: f64,
        num_changes: usize,
    ) -> Self {
        let mut changes: Vec<Box<dyn Change>> = Vec::new();
        for _ in 0..num_changes {
            let change = RandomMoveChange::new_random(rng, state, random_move_sigma);
            changes.push(Box::new(change));
        }
        Self { changes }
    }
}
