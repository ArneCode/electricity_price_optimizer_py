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
