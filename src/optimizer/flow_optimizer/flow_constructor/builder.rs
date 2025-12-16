use crate::optimizer::flow_constructor::MCMF::MinCostFlow;
use crate::optimizer::flow_constructor::helpers::INF;
use crate::optimizer::variable_maker::{self, VariableMaker};
use crate::optimizer_context::OptimizerContext;
use crate::time::{STEPS_PER_DAY, Time};

use super::action::construct_action;
use super::battery::construct_battery;
use super::consumption::add_beyond_control_consumption;
use super::helpers::calculate_total_flow;

pub fn contrusct_flow(context: &OptimizerContext) -> (MinCostFlow, VariableMaker) {
    let variable_map = VariableMaker::new(context);

    let WIRE = variable_maker::WIRE;

    let mut mcmf = MinCostFlow::new(
        variable_map.get_variable_count() as usize,
        variable_maker::SOURCE as usize,
        variable_maker::SINK as usize,
    );

    // Go from Source to Fork with Cost = 0, Capacity = total flow to complete tasks
    let total_flow = calculate_total_flow(context);
    mcmf.add_edge(
        variable_maker::SOURCE as usize,
        variable_maker::FORK_FROM_SOURCE as usize,
        total_flow,
        0,
    );

    // Seperate from fork to Network and Generator
    mcmf.add_edge(
        variable_maker::FORK_FROM_SOURCE as usize,
        variable_maker::NETWORK as usize,
        total_flow,
        0,
    );
    mcmf.add_edge(
        variable_maker::FORK_FROM_SOURCE as usize,
        variable_maker::GENERATOR as usize,
        total_flow,
        0,
    );

    // Generator to Wire
    let generator_prognoses = context.get_generated_electricity(); // TODO
    for t in 0..STEPS_PER_DAY {
        mcmf.add_edge(
            variable_maker::GENERATOR as usize,
            variable_map.get_wire_index(t).unwrap() as usize,
            *generator_prognoses.get(Time::from_timestep(t)).unwrap() as i64,
            0,
        );
    }

    // Network to Wire
    let network_prognoses = context.get_electricity_price();
    for t in 0..STEPS_PER_DAY {
        mcmf.add_edge(
            variable_maker::NETWORK as usize,
            variable_map.get_wire_index(t).unwrap() as usize,
            INF,
            *network_prognoses.get(Time::from_timestep(t)).unwrap() as i64,
        );
    }

    construct_battery(&mut mcmf, &variable_map, context);

    construct_action(&mut mcmf, &variable_map, context);

    add_beyond_control_consumption(&mut mcmf, &variable_map, context);

    return (mcmf, variable_map);
}