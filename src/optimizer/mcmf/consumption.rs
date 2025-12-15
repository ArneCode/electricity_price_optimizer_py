use crate::{
    optimizer::{
        mcmf::MinCostFlow,
        variable_maker::{self, VariableMaker},
    },
    optimizer_context::OptimizerContext,
    time::{STEPS_PER_DAY, Time},
};

use super::helpers::{INF, add_variable_capacity};

pub(crate) fn add_beyond_control_consumption(
    mf: &mut MinCostFlow,
    variable_map: &VariableMaker,
    context: &OptimizerContext,
) {
    let beyond_control = context.get_beyond_control_consumption();

    for t in 0..STEPS_PER_DAY {
        let wire_num = variable_map.get_wire_index(t).unwrap();

        mf.add_edge(
            wire_num as usize,
            variable_maker::SINK as usize,
            *beyond_control.get(Time::from_timestep(t)).unwrap() as i64,
            0,
        );
    }
}
