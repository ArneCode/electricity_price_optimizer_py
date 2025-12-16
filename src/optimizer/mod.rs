use std::{collections::HashMap, hash::Hash};

use std::time::Instant;

use crate::optimizer::flow_optimizer::flow::FlowWrapper;
use crate::optimizer_context::action::constant::ConstantAction;
use crate::optimizer_context::action::variable::VariableAction;
use crate::optimizer_context::battery::Battery;
use crate::optimizer_context::prognoses::Prognoses;
use crate::time::{STEPS_PER_DAY, Time};
use crate::schedule::Schedule;

mod flow_optimizer;

pub(crate) struct SmartHomeFlow {
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

        let SOURCE = flow.source();
        let SINK = flow.sink();
        let NETWORK = flow.add_node();
        let GENERATOR = flow.add_node();

        flow.add_edge(SOURCE, GENERATOR, i64::MAX, 0);
        flow.add_edge(GENERATOR, NETWORK, i64::MAX, 0);

        for i in 0..STEPS_PER_DAY {
            // Edge from SOURCE to wire for generation
            let gen_amount = *generate_prog.get(Time { minutes: i as u32 }).unwrap_or(&0) as i64;
            if gen_amount > 0 {
                flow.add_edge(GENERATOR, (0 as usize, i as usize), gen_amount, 0);
            }

            // Edge from wire to SINK for consumption
            let cons_amount = *consume_prog.get(Time { minutes: i as u32 }).unwrap_or(&0) as i64;
            flow.add_edge((0 as usize, i as usize), SINK, cons_amount, 0);

            // Edge from NETWORK to wire with cost based on price
            let price = *price_prog.get(Time { minutes: i as u32 }).unwrap_or(&0) as i64;
            flow.add_edge(NETWORK, (0 as usize, i as usize), i64::MAX, price);
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

    pub fn add_battery(&mut self, battery: Battery) {
        let id = battery.get_id();

        // Initialize battery
        let initial_level = battery.get_initial_level() as i64;
        self.flow.add_edge(
            self.SOURCE,
            (id as usize, 0 as usize),
            initial_level,
            0,
        );

        
        // Wire to Batteries
        for t in 0..STEPS_PER_DAY {
            // Wire to battery
            self.flow.add_edge(
                (0 as usize, t as usize),
                (id as usize, t as usize),
                battery.get_max_charge() as i64,
                0,
            );

            // Battery to wire
            self.flow.add_edge(
                (id as usize, t as usize),
                (0 as usize, t as usize),
                battery.get_max_output() as i64,
                0,
            );
        }

        // Battery persistence
        for t in 0..(STEPS_PER_DAY - 1) {
            self.flow.add_edge(
                (id as usize, t as usize),
                (id as usize, (t + 1) as usize),
                battery.get_capacity() as i64,
                0,
            );
        }
    }

    pub fn add_action(&mut self, variable_action: VariableAction) {
        let start = variable_action.get_start().minutes as usize;
        let end = variable_action.get_end().minutes as usize;
        for t in start..(end+1) {
            // Construct a Time value for this step if needed
            let _time = Time { minutes: t as u32 };
            // Wire to action
            self.flow.add_edge(
                (0 as usize, t as usize),
                (variable_action.get_id() as usize, 0 as usize),
                variable_action.get_max_consumption() as i64,
                0,
            );
        }

        // Action to Sink
        self.flow.add_edge(
            (variable_action.get_id() as usize, 0 as usize),
            self.SINK,
            variable_action.get_total_consumption() as i64,
            0,
        );
    }

    // Both functions work in progress:
    pub fn add_constant_consumption(&mut self, constant_action: ConstantAction) {
        
    }
    pub fn remove_constant_consumption(&mut self, constant_action: ConstantAction) {

    }

    pub fn calc_flow(&mut self) {
        let (flow_cost, flow_value) = self.flow.mincostflow();
        println!("Total flow: {}, Total cost: {}", flow_value, flow_cost);
    }
    // pub fn get_cost(&mut self) -> Option<Cost> {}
    // pub fn construct_schedule() -> Option<Schedule> {}
}
