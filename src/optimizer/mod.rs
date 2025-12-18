use std::collections::HashSet;
use std::{collections::HashMap, hash::Hash};

use std::time::Instant;

use crate::helper::stack_proxy::StackProxy;
use crate::optimizer::flow_optimizer::flow::FlowWrapper;
use crate::optimizer::flow_optimizer::flow::wrapper::FlowNode;
use crate::optimizer_context::action::constant::{self, AssignedConstantAction, ConstantAction};
use crate::optimizer_context::action::variable::VariableAction;
use crate::optimizer_context::battery::Battery;
use crate::optimizer_context::prognoses::Prognoses;
use crate::time::{STEPS_PER_DAY, Time};
use crate::schedule::Schedule;

mod flow_optimizer;
pub struct SmartHomeFlowBuilder {
    flow: FlowWrapper,
}
impl SmartHomeFlowBuilder {
    pub fn new(
        generate_prog: &Prognoses<i32>,
        price_prog: &Prognoses<i32>,
        consume_prog: &Prognoses<i32>,
    ) -> Self {
        let mut flow = FlowWrapper::new();

        flow.add_edge(FlowNode::Source, FlowNode::Generator, i64::MAX, 0);
        flow.add_edge(FlowNode::Source, FlowNode::Network, i64::MAX, 0);

        for i in 0..STEPS_PER_DAY {
            // Edge from GENERATOR to wire for generation
            let gen_amount = *generate_prog.get(Time::from_timestep(i)).unwrap_or(&0) as i64;
            if gen_amount > 0 {
                flow.add_edge(FlowNode::Generator, FlowNode::Wire(Time::from_timestep(i)), gen_amount, 0);
            }

            // Edge from NETWORK to wire with cost based on price
            let price = *price_prog.get(Time::from_timestep(i)).unwrap_or(&0) as i64;
            flow.add_edge(FlowNode::Network, FlowNode::Wire(Time::from_timestep(i)), i64::MAX, price);

            // Edge from wire to SINK for consumption
            let cons_amount = *consume_prog.get(Time::from_timestep(i)).unwrap_or(&0) as i64;
            if cons_amount > 0 {
                flow.add_edge(FlowNode::Wire(Time::from_timestep(i)), FlowNode::Sink, cons_amount, 0);
            }
        }

        Self {
            flow,
        }
    }

    pub fn add_battery(mut self, battery: &Battery) -> Self {
        let id = battery.get_id();

        // Initialize battery
        let initial_level = battery.get_initial_level() as i64;
        self.flow.add_edge(
            FlowNode::Source,
            FlowNode::Battery(id as usize, Time::from_timestep(0)),
            initial_level,
            0,
        );

        
        // Wire to Batteries
        for t in 0..STEPS_PER_DAY {
            // Wire to battery
            self.flow.add_edge(
                FlowNode::Wire(Time::from_timestep(t)),
                FlowNode::Battery(id as usize, Time::from_timestep(t)),
                battery.get_max_charge() as i64,
                0,
            );

            // Battery to wire
            self.flow.add_edge(
                FlowNode::Battery(id as usize, Time::from_timestep(t)),
                FlowNode::Wire(Time::from_timestep(t)),
                battery.get_max_output() as i64,
                0,
            );
        }

        // Battery persistence
        for t in 0..(STEPS_PER_DAY - 1) {
            self.flow.add_edge(
                FlowNode::Battery(id as usize, Time::from_timestep(t)),
                FlowNode::Battery(id as usize, Time::from_timestep(t + 1)),
                battery.get_capacity() as i64,
                0,
            );
        }
        self
    }

    pub fn add_batteries(mut self, batteries: &Vec<Battery>) -> Self {
        for battery in batteries {
            self = self.add_battery(battery);
        }
        self
    }
    pub fn add_action(mut self, variable_action: &VariableAction) -> Self{
        let start = variable_action.get_start().to_timestep() as usize;
        let end = variable_action.get_end().to_timestep() as usize;
        for t in start..end {
            // Construct a Time value for this step if needed
            let _time = Time::from_timestep(t as u32);
            // Wire to action
            self.flow.add_edge(
                FlowNode::Wire(Time::from_timestep(t as u32)),
                FlowNode::Action(variable_action.get_id() as usize),
                variable_action.get_max_consumption() as i64,
                0,
            );
        }

        // Action to Sink
        self.flow.add_edge(
            FlowNode::Action(variable_action.get_id() as usize),
            FlowNode::Sink,
            variable_action.get_total_consumption() as i64,
            0,
        );
        self
    }
    pub fn add_actions(mut self, variable_actions: &Vec<VariableAction>) -> Self {
        for action in variable_actions {
            self = self.add_action(action);
        }
        self
    }
    pub fn build(mut self) -> SmartHomeFlow {
        self.flow.mincostflow();
        SmartHomeFlow::new(
            self.flow
        )
    }
}
pub(crate) struct SmartHomeFlow {
    flow: StackProxy<FlowWrapper>,

    constant_actions: HashSet<AssignedConstantAction>,

    calc_result: Option<i64>,
    
    schedule_relevant_edges: Vec<(usize, Box<dyn Fn(i32, &mut Schedule)>)>,
}

// WARNING: wire has ID = 0, make sure no node uses this ID!
impl SmartHomeFlow {
        pub fn new(
        flow: FlowWrapper
    ) -> Self {
        let flow: StackProxy<FlowWrapper> = StackProxy::new(flow);
        SmartHomeFlow {
            flow,
            constant_actions: HashSet::new(),
            calc_result: None,
            schedule_relevant_edges: Vec::new(),
        }
    }

    // Both functions work in progress:
    pub fn add_constant_consumption(&mut self, constant_action: AssignedConstantAction) {
        self.constant_actions.insert(constant_action);
        self.calc_result = None;
    }
    pub fn remove_constant_consumption(&mut self, constant_action: AssignedConstantAction) {
        self.constant_actions.remove(&constant_action);
        self.calc_result = None;
    }

    pub fn calc_flow(&mut self) {
        self.flow.push();

        for constant_action in &self.constant_actions {
            let start = constant_action.get_start_from().to_timestep() as usize;
            let end = constant_action.get_end_before().to_timestep() as usize;
            for t in start..end {
                // Wire to sink
                self.flow.add_edge(
                    FlowNode::Wire(Time::from_timestep(t as u32)),
                    FlowNode::Sink,
                    constant_action.get_consumption() as i64,
                    0,
                );
            }
        }

        let (flow_cost, flow_value) = self.flow.mincostflow();
        self.calc_result = Some(flow_cost);
        println!("Total flow: {}, Total cost: {}", flow_value, flow_cost);
        self.flow.pop();
    }
    pub fn get_cost(&self) -> Option<i64> {
        self.calc_result
    }
    // pub fn get_cost(&mut self) -> Option<Cost> {}
    // pub fn construct_schedule() -> Option<Schedule> {}
}

/*
let builder = SmartHomeFlowBuilder::new(
    generate_prog,
    price_prog,
    consume_prog,
)
.add_battery(battery1)
.add_battery(battery2)
.add_variable_action(variable_action1);
let smart_home_flow = builder.build();
*/