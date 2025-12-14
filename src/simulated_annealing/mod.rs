use rand::Rng;

use crate::{
    optimizer::get_cost,
    optimizer_context::OptimizerContext,
    simulated_annealing::{
        change::{Change, multi_change::MultiChange},
        state::State,
    },
};

mod change;
pub mod state;

pub fn run_simulated_annealing(context: OptimizerContext) -> i64 {
    let mut state = State::new(context);
    let mut temperature: f64 = 1000.0;

    let mut rng = rand::rng();

    // TODO: actually calculate this:
    let mut old_cost = get_cost(&state.to_fixed_context());
    while temperature > 0.1 {
        let change = MultiChange::new_random(&mut rng, &state, 1.0, 2);
        change.apply(&mut state);
        // Evaluate the new state and decide whether to accept or reject the change
        // TODO: actually calculate this:
        let new_cost = get_cost(&state.to_fixed_context());
        let cost_diff = new_cost - old_cost;
        if cost_diff < 0 {
            // Accept the change
            old_cost = new_cost;
        } else {
            let acceptance_probability = (-cost_diff as f64 / temperature).exp();
            if rng.random_range(0.0..1.0) < acceptance_probability {
                // Accept the change
                old_cost = new_cost;
            } else {
                // Reject the change
                change.undo(&mut state);
            }
        }
        temperature *= 0.99; // Cool down
        println!("temperature: {temperature}, cost: {old_cost}");
    }

    old_cost

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
        let mut electricity_price_data = [100; 1440];
        electricity_price_data[0] = 10;
        let generated_electricity_data = [0; 1440];
        let beyond_control_consumption_data = [0; 1440];
        let batteries = vec![Battery::new(10, 0, 10, 7, 1.0, 1)];
        let constant_actions = vec![Rc::new(ConstantAction::new(0, 10, 2, 100, 2))]; // no use
        let variable_actions = vec![Rc::new(VariableAction::new(0, 10, 40, 10, 3))];

        let context = OptimizerContext::new(
            Prognoses::new(electricity_price_data),
            Prognoses::new(generated_electricity_data),
            Prognoses::new(beyond_control_consumption_data),
            batteries,
            constant_actions,
            variable_actions,
        ); // Assuming a constructor exists
        let result = run_simulated_annealing(context);
        println!("result: {result}");
        // Add assertions to verify the results
    }
}
