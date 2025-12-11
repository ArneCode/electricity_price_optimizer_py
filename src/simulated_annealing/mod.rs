use rand::Rng;

use crate::{
    optimizer_context::OptimizerContext,
    simulated_annealing::{
        change::{Change, multi_change::MultiChange},
        state::State,
    },
};

mod change;
pub mod state;

pub fn run_simulated_annealing(context: OptimizerContext) {
    let mut state = State::new(context);
    let mut temperature: f64 = 1000.0;

    let mut rng = rand::rng();

    // TODO: actually calculate this:
    let old_cost = rng.random_range(0.0..100.0);
    while temperature > 0.1 {
        let change = MultiChange::new_random(&mut rng, &state, 1.0, 2);
        change.apply(&mut state);
        // Evaluate the new state and decide whether to accept or reject the change
        // TODO: actually calculate this:
        let new_cost = rng.random_range(0.0..100.0);
        let cost_diff = new_cost - old_cost;
        if cost_diff < 0.0 {
            // Accept the change
        } else {
            let acceptance_probability = (-cost_diff / temperature).exp();
            if rng.random_range(0.0..1.0) < acceptance_probability {
                // Accept the change
            } else {
                // Reject the change
                change.undo(&mut state);
            }
        }
        temperature *= 0.99; // Cool down
    }

    // somehow also get the final schedule out of the state
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rand::rand_core::le;

    use crate::optimizer_context::{
        action::{
            constant::{self, ConstantAction},
            variable::{self, VariableAction},
        },
        battery::Battery,
        prognoses::Prognoses,
    };

    use super::*;

    #[test]
    fn test_simulated_annealing() {
        let electricity_price_data = [10; 1440];
        let generated_electricity_data = [5; 1440];
        let beyond_control_consumption_data = [20; 1440];
        let batteries = vec![Battery::new(1000, 10, 10, 7, 1.0, 1)];
        let constant_actions = vec![Rc::new(ConstantAction::new(0, 60, 120, 15, 2))];
        let variable_actions = vec![Rc::new(VariableAction::new(1, 200, 300, 50, 3))];

        let context = OptimizerContext::new(
            Prognoses::new(electricity_price_data),
            Prognoses::new(generated_electricity_data),
            Prognoses::new(beyond_control_consumption_data),
            batteries,
            constant_actions,
            variable_actions,
        ); // Assuming a constructor exists
        run_simulated_annealing(context);
        // Add assertions to verify the results
    }
}
