use crate::{
    optimizer::{
        mcmf::{
            MinCostFlow,
            helpers::{INF, add_variable_capacity},
        },
        variable_maker::{self, VariableMaker},
    },
    optimizer_context::OptimizerContext,
    time::STEPS_PER_DAY,
};

pub(crate) fn construct_battery(
    mf: &mut MinCostFlow,
    variable_map: &VariableMaker,
    context: &OptimizerContext,
) {
    for b in context.get_batteries() {
        let id = b.get_id();

        // Initialize battery
        let first_battery_incoming_num =
            variable_map.get_persistent_variable_with_capacity_index(id, 0, true);
        let initial_level = b.get_initial_level() as i64;
        mf.add_edge(
            variable_maker::SOURCE as usize,
            first_battery_incoming_num.unwrap() as usize,
            initial_level,
            0,
        );

        add_variable_capacity(id, mf, variable_map, b.get_capacity() as i64);

        // Wire to Batteries
        for t in 0..STEPS_PER_DAY {
            let battery_incoming_num =
                variable_map.get_persistent_variable_with_capacity_index(id, t, true);
            let battery_outgoing_num =
                variable_map.get_persistent_variable_with_capacity_index(id, t, false);
            let wire_num = variable_map.get_wire_index(t).unwrap();

            // Wire to battery
            mf.add_edge(
                wire_num as usize,
                battery_incoming_num.unwrap() as usize,
                b.get_max_charge() as i64,
                0,
            );

            // Battery to wire
            mf.add_edge(
                battery_outgoing_num.unwrap() as usize,
                wire_num as usize,
                b.get_max_output() as i64,
                0,
            );
        }

        // Battery persistence
        for t in 0..(STEPS_PER_DAY - 1) {
            let battery_current_num =
                variable_map.get_persistent_variable_with_capacity_index(id, t, false);
            let battery_next_num =
                variable_map.get_persistent_variable_with_capacity_index(id, t + 1, true);

            mf.add_edge(
                battery_current_num.unwrap() as usize,
                battery_next_num.unwrap() as usize,
                INF,
                0,
            );
        }
    }
}
