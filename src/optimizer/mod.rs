mod mcmf;
mod variable_maker;

use std::time::Instant;

use mcmf::helpers::calculate_mcmf_cost;

use crate::optimizer_context::OptimizerContext;

pub fn get_cost(context: &OptimizerContext) -> i64 {
    let start = Instant::now();
    let cost = calculate_mcmf_cost(context);
    let elapsed = start.elapsed();
    println!("flow took: {elapsed:.2?}");
    return cost;
}
pub fn get_construction() {} // TODO

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
    fn test_flow() {
        let mut electricity_price_data = [100; 1440];
        electricity_price_data[0] = 10;
        let generated_electricity_data = [0; 1440];
        let beyond_control_consumption_data = [0; 1440];
        let batteries = vec![Battery::new(10, 0, 10, 7, 1.0, 1)];
        let constant_actions = vec![Rc::new(ConstantAction::new(0, 10, 6, 100, 2))]; // no use
        let variable_actions = vec![Rc::new(VariableAction::new(0, 10, 40, 10, 3))];

        let context = OptimizerContext::new(
            Prognoses::new(electricity_price_data),
            Prognoses::new(generated_electricity_data),
            Prognoses::new(beyond_control_consumption_data),
            batteries,
            constant_actions,
            variable_actions,
        ); // Assuming a constructor exists
        let result = get_cost(&context);
        println!("result: {result}");
        // Add assertions to verify the results
    }
}