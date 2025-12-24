use std::collections::{HashMap, HashSet};

use crate::{
    optimizer::{SmartHomeFlow, SmartHomeFlowBuilder},
    optimizer_context::{
        OptimizerContext,
        action::{
            constant::{self, AssignedConstantAction},
            variable::VariableAction,
        },
    },
    time::Time,
};

pub struct State {
    constant_actions: HashMap<u32, AssignedConstantAction>,
    constant_action_ids: Vec<u32>,

    smart_home_flow: SmartHomeFlow,

    is_valid: bool,
}

impl State {
    pub fn new(context: OptimizerContext) -> Self {
        let constant_actions: HashMap<u32, AssignedConstantAction> = context
            .get_constant_actions()
            .iter()
            .map(|action| {
                // let start_minutes = action.get_start_from().get_minutes();
                // let end_minutes =
                //     action.get_end_before().get_minutes() - action.duration.get_minutes();
                // let middle_minutes = (start_minutes + end_minutes) / 2;
                // AssignedConstantAction::new(action.clone(), Time::new(0, middle_minutes))
                (
                    action.get_id(),
                    AssignedConstantAction::new(action.clone(), action.get_start_from()),
                )
            })
            .collect();
        let mut smart_home_flow = SmartHomeFlowBuilder::new(
            context.get_generated_electricity(),
            context.get_beyond_control_consumption(),
            context.get_electricity_price(),
        )
        .add_batteries(context.get_batteries())
        .add_actions(context.get_variable_actions())
        .build();

        for (_, action) in constant_actions.iter() {
            smart_home_flow.add_constant_consumption(action.clone());
        }

        let constant_action_ids = constant_actions.keys().cloned().collect();

        Self {
            constant_actions,
            constant_action_ids,
            smart_home_flow,
            is_valid: false,
        }
    }
    pub fn add_constant_action(&mut self, action: AssignedConstantAction) {
        self.smart_home_flow
            .add_constant_consumption(action.clone());
        self.constant_actions.insert(action.get_id(), action);
        self.is_valid = false;
    }
    pub fn remove_constant_action(&mut self, action_id: u32) -> Option<AssignedConstantAction> {
        self.is_valid = false;
        self.constant_actions.remove(&action_id);
        self.smart_home_flow.remove_constant_consumption(action_id)
    }

    pub fn get_constant_action(&self, action_id: u32) -> &AssignedConstantAction {
        self.constant_actions.get(&action_id).unwrap()
    }

    pub fn get_constant_action_ids(&self) -> &Vec<u32> {
        &self.constant_action_ids
    }

    pub fn run_local_search(&mut self) {
        self.smart_home_flow.calc_flow();
        self.is_valid = true;
    }

    pub fn get_cost(&self) -> i64 {
        assert!(self.is_valid);
        self.smart_home_flow.get_cost().unwrap()
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    // pub fn to_fixed_context(&self) -> OptimizerContext {
    //     let mut new_context = self.context.clone();
    //     for action in &self.constant_actions {
    //         new_context.add_constant_action_to_consumption(action);
    //     }
    //     new_context
    // }
}
