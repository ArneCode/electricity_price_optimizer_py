use std::{collections::HashMap, hash::Hash};

use std::time::Instant;

use crate::optimizer_context::action::constant::ConstantAction;
use crate::optimizer_context::action::variable::VariableAction;
use crate::optimizer_context::battery::Battery;
use crate::optimizer_context::prognoses::Prognoses;
use crate::time::{STEPS_PER_DAY, Time};
use crate::optimizer::flow_optimizer::flow::FlowWrapper;
use crate::schedule::Schedule;

mod flow_optimizer;

struct SmartHomeFlow {
    flow: FlowWrapper,
    
    schedule_relevant_edges: Vec<(usize, Box<dyn Fn(i32, &mut Schedule)>)>,

    SOURCE: usize,
    SINK: usize,
    NETWORK: usize,
    GENERATOR: usize,
}

// WARNING: wire has ID = 0, make sure no node uses this ID!
impl SmartHomeFlow {
        pub fn new(
        generate_prog: Prognoses<i32>, 
        price_prog: Prognoses<i32>, 
        consume_prog: Prognoses<i32>
    ) -> Self {
        let mut flow = FlowWrapper::new();

        let SOURCE = flow.get_source();
        let SINK = flow.get_sink();
        let NETWORK = flow.add_node();
        let GENERATOR = flow.add_node();

        flow.add_edge(SOURCE, GENERATOR, i64::MAX, 0);
        flow.add_edge(GENERATOR, NETWORK, i64::MAX, 0);

        for i in 0..STEPS_PER_DAY {
            // Edge from SOURCE to wire for generation
            let gen_amount = *generate_prog.get(Time { minutes: i as u32 }).unwrap_or(&0) as i64;
            if gen_amount > 0 {
                flow.add_edge(GENERATOR, (0, i), gen_amount, 0);
            }

            // Edge from wire to SINK for consumption
            let cons_amount = *consume_prog.get(Time { minutes: i as u32 }).unwrap_or(&0) as i64;
            flow.add_edge((0, i), SINK, cons_amount, 0);

            // Edge from NETWORK to wire with cost based on price
            let price = *price_prog.get(Time { minutes: i as u32 }).unwrap_or(&0) as i64;
            flow.add_edge(NETWORK, (0, i), i64::MAX, price);
        }

        SmartHomeFlow {
            flow,
            schedule_relevant_edges: Vec::new(),
            SOURCE,
            SINK,
            NETWORK,
            GENERATOR,
        }
    }

    pub fn add_battery(&mut self, _battery: Battery) {
        // TODO: integrate the battery into the flow graph
    }

    pub fn add_action(&mut self, _variable_action: VariableAction) {
        // TODO: translate the variable action into flow edges/constraints
    }
    pub fn add_constant_consumption(&mut self, constant_action: ConstantAction) {
        
    }
    pub fn remove_constant_consumption(&mut self, constant_action: ConstantAction) {

    }

    // pub fn calc_flow(&mut self) {
    //     let (flow_cost, flow_value) = self.flow.mincostflow();
    //     println!("Total flow: {}, Total cost: {}", flow_value, flow_cost);
    // }
    // pub fn get_cost(&mut self) -> Option<Cost> {}
    // pub fn construct_schedule() -> Option<Schedule> {}
}
