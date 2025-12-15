// optimizer/mcmf/helpers.rs
// pub const MINUTES_PER_DAY: u32 = 60 * 24;
pub const INF: i64 = 1_i64 << 60;

use crate::optimizer::mcmf::MinCostFlow;
use crate::optimizer::mcmf::builder::contrusct_flow;
use crate::optimizer::variable_maker::VariableMaker;
use crate::optimizer_context::OptimizerContext;
use crate::time::STEPS_PER_DAY;

pub fn calculate_total_flow(context: &OptimizerContext) -> i64 {
    let mut total = 0;
    for action in context.get_variable_actions() {
        total += action.get_total_consumption() as i64;
    }

    total += context
        .get_beyond_control_consumption()
        .get_data()
        .iter()
        .to_owned()
        .sum::<i32>() as i64;
    return total;
}

pub fn add_variable_capacity(
    item_id: i32,
    mf: &mut MinCostFlow,
    variable_map: &VariableMaker,
    cap: i64,
) {
    for t in 0..STEPS_PER_DAY {
        mf.add_edge(
            variable_map
                .get_persistent_variable_with_capacity_index(item_id, t, true)
                .unwrap() as usize,
            variable_map
                .get_persistent_variable_with_capacity_index(item_id, t, false)
                .unwrap() as usize,
            cap,
            0,
        );
    }
}

pub(crate) fn calculate_mcmf_cost(context: &OptimizerContext) -> i64 {
    let (mut mcmf, _variable_map) = contrusct_flow(context);
    let (maxflow, mincost) = mcmf.mincostflow();
    if (maxflow as i64) < calculate_total_flow(context) {
        panic!("Could not satisfy all flows in MCMF construction");
    }
    return mincost;
}
