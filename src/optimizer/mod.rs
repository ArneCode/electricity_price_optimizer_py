mod MCMF;
mod variable_maker;

use crate::optimizer::MCMF::MinCostFlow;
use crate::optimizer::variable_maker::VariableMaker;
use crate::simulated_annealing::state::State;

pub const MINUTES_PER_DAY: u32 = 60 * 24;

struct optimizer {
    pub mf:MinCostFlow,
    pub variable_map:VariableMaker,
}

impl optimizer {
    pub fn new(state: &State) -> Self {
        let variable_map = VariableMaker::new(state.get_context());

        let WIRE = variable_maker::WIRE;

        let mut mf = MinCostFlow::new(variable_map.get_variable_count() as usize, variable_maker::SOURCE as usize, variable_maker::SINK as usize);
    
        // Go from Source to Fork with Cost = 0, Capacity = total flow to complete tasks
        let total_flow = 64; // to change
        mf.add_edge(variable_maker::SOURCE as usize, variable_maker::FORK_FROM_SOURCE as usize, total_flow, 0);

        // Seperate from fork to Network and Generator
        mf.add_edge(variable_maker::FORK_FROM_SOURCE as usize, variable_maker::NETWORK as usize, total_flow, 0);
        mf.add_edge(variable_maker::FORK_FROM_SOURCE as usize, variable_maker::GENERATOR as usize, total_flow, 0);

        // Generator to System flow
        for t in 0..MINUTES_PER_DAY {

        }

        Self {
            mf:mf,
            variable_map:variable_map,
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
}
