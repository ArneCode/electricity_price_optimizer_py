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

pub(crate) fn construct_action(
    mf: &mut MinCostFlow,
    variable_map: &VariableMaker,
    context: &OptimizerContext,
) {
    for a in context.get_variable_actions() {
        let id = a.get_id() as i32;
        let task_start = a.get_start().to_timestep();
        let task_end = a.get_end().to_timestep();

        // Wire to Actions
        for t in task_start..(task_end+1) {
            let action_incoming_num = variable_map.get_persistent_variable_index(id, t);
            let action_max_consumption = a.get_max_consumption() as i64;

            mf.add_edge(
                variable_map.get_wire_index(t).unwrap() as usize,
                action_incoming_num.unwrap() as usize,
                action_max_consumption,
                0,
            );
        }

        // Action persistence
        for t in task_start..task_end {
            let action_current_num = variable_map.get_persistent_variable_index(id, t);
            let action_next_num = variable_map.get_persistent_variable_index(id, t + 1);

            mf.add_edge(
                action_current_num.unwrap() as usize,
                action_next_num.unwrap() as usize,
                INF,
                0,
            );
        }

        let action_end_num = variable_map.get_persistent_variable_index(id, task_end);
        mf.add_edge(
            action_end_num.unwrap() as usize,
            variable_maker::SINK as usize,
            a.get_total_consumption() as i64,
            0,
        );
    }
}
