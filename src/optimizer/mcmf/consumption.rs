use crate::{optimizer::{mcmf::MinCostFlow, variable_maker::{self, VariableMaker}}, optimizer_context::OptimizerContext};

use super::helpers::{add_variable_capacity, MINUTES_PER_DAY, INF};

pub(crate) fn add_beyond_control_consumption(
    mf: &mut MinCostFlow,
    variable_map: &VariableMaker,
    context: &OptimizerContext,
) {
    let beyond_control = context.get_beyond_control_consumption();

    for t in 0..MINUTES_PER_DAY {
        let wire_num = variable_map.get_wire_index(t).unwrap();

        mf.add_edge(
            wire_num as usize,
            variable_maker::SINK as usize,
            *beyond_control.get(t as usize).unwrap() as i64,
            0,
        );
    }
}