mod mcmf;
mod variable_maker;

use mcmf::helpers::calculate_mcmf_cost;

use crate::optimizer_context::OptimizerContext;

pub fn get_cost(context: &OptimizerContext) -> i64 {
  return calculate_mcmf_cost(context);
}
pub fn get_construction() {} // TODO