mod mcmf;
use std::{collections::HashMap, hash::Hash};

use std::time::Instant;

use mcmf::helpers::calculate_mcmf_cost;

use crate::{
    optimizer::mcmf::FlowWrapper, schedule::Schedule,
};

pub type PersistentVariableIndex = (i32, u32);
struct SmartHomeFlow {
    flow: FlowWrapper,
    
    schedule_relevant_edges: Vec<(usize, Box<dyn Fn(i32, &mut Schedule)>)>,

    FLOW: usize,
    SINK: usize,
    NETWORK: usize,
    GENERATOR: usize,
}

impl SmartHomeFlow {
    // pub fn new(generate_prog, price_prog, consume_prog) -> Self;
    // pub fn add_battery(battery);
    // pub fn add_action(variable_action);
    // pub fn add_constant_consumption(constant_action);
    // pub fn remove_constant_consumption(constant_action);

    // pub fn calc_flow();
    // pub fn get_cost() -> Option<Cost>;
    // pub fn construct_schedule() -> Option<Schedule>;
}

struct SimulatedAnnealingState {
    constant_actions: Vec<AssignedConstantAction>,
    flow: SmartHomeFlow
}
