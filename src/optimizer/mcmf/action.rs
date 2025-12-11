use crate::{optimizer::{mcmf::{MinCostFlow, helpers::{INF, MINUTES_PER_DAY, add_variable_capacity}}, variable_maker::{self, VariableMaker}}, optimizer_context::OptimizerContext};

pub(crate) fn construct_action(
    mf: &mut MinCostFlow,
    variable_map: &VariableMaker,
    context: &OptimizerContext,
) {
    for a in context.get_variable_actions() {
        let id = a.get_id() as i32;

        add_variable_capacity(id, mf, variable_map, a.get_total_consumption() as i64);

        // Wire to Actions
        for t in 0..MINUTES_PER_DAY {
            let action_incoming_num = variable_map.get_persistent_variable_index(id, t, true);
            let action_max_consumption = a.get_max_consumption() as i64;

            mf.add_edge(
                variable_map.get_wire_index(t).unwrap() as usize,
                action_incoming_num.unwrap() as usize,
                action_max_consumption,
                0,
            );
        }

        // Action persistence
        for t in 0..(MINUTES_PER_DAY - 1) {
            let action_current_num = variable_map.get_persistent_variable_index(id, t, false);
            let action_next_num = variable_map.get_persistent_variable_index(id, t + 1, true);

            mf.add_edge(
                action_current_num.unwrap() as usize,
                action_next_num.unwrap() as usize,
                INF,
                0,
            );
        }

        let action_end_num = variable_map.get_persistent_variable_index(id, MINUTES_PER_DAY - 1, false);
        mf.add_edge(
            action_end_num.unwrap() as usize,
            variable_maker::SINK as usize,
            INF,
            0,
        );
    }
}