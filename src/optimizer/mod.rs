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
        let variable_maker = VariableMaker::new(state.get_context());

        let SYSTEM_FLOW = variable_maker::SYSTEM_FLOW;

        let mf = MinCostFlow::new(variable_maker.get_variable_count(), variable_maker::SOURCE, variable_maker::SINK);
    
        mf.add_edge(variable_maker::SOURCE, variable_maker::NETWORK, 0, 0);
        mf.add_edge(variable_maker::SOURCE, variable_maker::GENERATOR, 0, 0);

        for t in 0..MINUTES_PER_DAY {
            mf.add_edge(variable_maker::SOURCE.try_into().unwrap(), variable_maker.get_system_flow_index(SYSTEM_FLOW, t), 0, 0);
        }
    }
}
