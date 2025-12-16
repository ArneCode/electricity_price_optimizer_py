use crate::{
    optimizer::{
        SmartHomeFlow, mcmf::{
            MinCostFlow,
            helpers::{INF},
        }, variable_maker::{self, VariableMaker}
    },
    optimizer_context::{OptimizerContext, battery::Battery},
    time::STEPS_PER_DAY,
};
impl SmartHomeFlow {
    pub fn add_battery(&mut self, battery: Battery) {
        let id = battery.get_id();

        // Initialize battery
        let first_battery_incoming_num = self.persistent_variable_indices.get(&(id, 0)).r;
        let initial_level = battery.get_initial_level() as i64;
        let ind = self.flow.add_edge(
            variable_maker::SOURCE as usize,
            first_battery_incoming_num,
            initial_level,
            0,
        );

        for t in 0..STEPS_PER_DAY {
            
            flow.add_edge(
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