mod MCMF;
mod variable_maker;

use crate::optimizer::MCMF::MinCostFlow;
use crate::optimizer::variable_maker::VariableMaker;
use crate::optimizer_context::OptimizerContext;
use crate::simulated_annealing::state::State;

pub const MINUTES_PER_DAY: u32 = 60 * 24;
const INF: i64 = 1_i64 << 60;
    
pub fn contrusct_flow(context: &OptimizerContext) -> (MinCostFlow, VariableMaker) {
    let variable_map = VariableMaker::new(context);

    let WIRE = variable_maker::WIRE;

    let mut mf = MinCostFlow::new(variable_map.get_variable_count() as usize, variable_maker::SOURCE as usize, variable_maker::SINK as usize);
    
    // Go from Source to Fork with Cost = 0, Capacity = total flow to complete tasks
    let total_flow = 64; // to change
    mf.add_edge(variable_maker::SOURCE as usize, variable_maker::FORK_FROM_SOURCE as usize, total_flow, 0);

    // Seperate from fork to Network and Generator
    mf.add_edge(variable_maker::FORK_FROM_SOURCE as usize, variable_maker::NETWORK as usize, total_flow, 0);
    mf.add_edge(variable_maker::FORK_FROM_SOURCE as usize, variable_maker::GENERATOR as usize, total_flow, 0);

    // Generator to Wire
    let generator_prognoses = context.get // TODO
    for t in 0..MINUTES_PER_DAY {
        mf.add_edge(variable_maker::GENERATOR as usize, variable_map.get_wire_index(t).unwrap() as usize, , 0);
    }

    // Network to Wire
    let network_prognoses = context.get // TODO
    for t in 0..MINUTES_PER_DAY {
        mf.add_edge(variable_maker::NETWORK as usize, variable_map.get_wire_index(t).unwrap() as usize, INF, );
    }

    construct_battery(&mut mf, &mut variable_map, context);
    
    return (mf, variable_map);
}

fn construct_battery(mf: &mut MinCostFlow, variable_map: &mut VariableMaker, context: &OptimizerContext) {
    for b in context.get_batteries() {
        let id = b.get_id();
        // Wire to Batteries
        for t in 0..MINUTES_PER_DAY {
            let battery_incoming_num = variable_map.get_battery_variable_index(id, t, true);
            let battery_outgoing_num = variable_map.get_battery_variable_index(id, t, false);
            let wire_num = variable_map.get_wire_index(t).unwrap();
            
            // Wire to battery
            mf.add_edge(wire_num as usize, battery_incoming_num.unwrap() as usize, b.get_max_charge() as i64, 0);

            // Battery to wire
            mf.add_edge(battery_outgoing_num.unwrap() as usize, wire_num as usize, b.get_max_output() as i64, 0);
        }

        // Battery persistence
        for t in 0..(MINUTES_PER_DAY - 1) {
            let battery_current_num = variable_map.get_battery_variable_index(id, t, false);
            let battery_next_num = variable_map.get_battery_variable_index(id, t, true);
            
            mf.add_edge(battery_current_num.unwrap() as usize, battery_next_num.unwrap() as usize, INF, 0);
        }
    }
}

fn construct_action(mf: &mut MinCostFlow, variable_map: &mut VariableMaker, context: &OptimizerContext) {
    for a in context.get_variable_actions() {
        let id = a.get_id() as i32;
        // Wire to Actions
        for t in 0..MINUTES_PER_DAY {
            let action_incoming_num = variable_map.get_action_variable_index(id,t, true);
            let action_outgoing_num = variable_map.get_action_variable_index(id,t, false);

            mf.add_edge(variable_map.get_wire_index(t).unwrap() as usize, action_incoming_num.unwrap() as usize, INF, 0);
        }

        // Action persistence
        for t in 0..(MINUTES_PER_DAY - 1) {
            let action_current_num = variable_map.get_action_variable_index(id, t, false);
            let action_next_num = variable_map.get_action_variable_index(id, t, true);
            
            mf.add_edge(action_current_num.unwrap() as usize, action_next_num.unwrap() as usize, INF, 0);
        }
    }
}

fn add_action_variable_capacity(item_id: i32, mf: &mut MinCostFlow, variable_map: &mut VariableMaker, cap: i64) {
    for t in 0..MINUTES_PER_DAY {
        mf.add_edge(variable_map.get_action_variable_index(item_id, t, true).unwrap() as usize, variable_map.get_action_variable_index(item_id , t, false).unwrap() as usize, cap, 0);
    }
}
fn add_battery_capacity(item_id: i32, mf: &mut MinCostFlow, variable_map: &mut VariableMaker, cap: i64) {
    for t in 0..MINUTES_PER_DAY {
        mf.add_edge(variable_map.get_battery_variable_index(item_id, t, true).unwrap() as usize, variable_map.get_battery_variable_index(item_id , t, false).unwrap() as usize, cap, 0);
    }
}
