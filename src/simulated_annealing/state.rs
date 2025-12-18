use crate::{
    optimizer::{SmartHomeFlow, SmartHomeFlowBuilder}, optimizer_context::{
        OptimizerContext,
        action::{
            constant::{self, AssignedConstantAction},
            variable::VariableAction,
        },
    }, time::Time
};

pub struct State {
    constant_actions: Vec<AssignedConstantAction>,

    smart_home_flow: SmartHomeFlow,

    is_valid: bool,
}

impl State {
    pub fn new(context: OptimizerContext) -> Self {
        let constant_actions: Vec<AssignedConstantAction> = context
            .get_constant_actions()
            .iter()
            .map(|action| {
                // let start_minutes = action.get_start_from().get_minutes();
                // let end_minutes =
                //     action.get_end_before().get_minutes() - action.duration.get_minutes();
                // let middle_minutes = (start_minutes + end_minutes) / 2;
                // AssignedConstantAction::new(action.clone(), Time::new(0, middle_minutes))
                AssignedConstantAction::new(action.clone(), action.get_start_from())
            })
            .collect();
        let mut smart_home_flow = SmartHomeFlowBuilder::new(context.get_generated_electricity(), context.get_beyond_control_consumption(), context.get_electricity_price())
            .add_batteries(context.get_batteries())
            .add_actions(context.get_variable_actions())
            .build();

        for action in constant_actions.iter().cloned() {
            smart_home_flow.add_constant_consumption(action);
        }
    
        Self {
            constant_actions,
            smart_home_flow,
            is_valid: false,
        }
    }
    pub fn get_constant_actions(&self) -> &Vec<AssignedConstantAction> {
        &self.constant_actions
    }
    pub fn get_constant_actions_mut(&mut self) -> &mut Vec<AssignedConstantAction> {
        &mut self.constant_actions
    }

    pub fn set_constant_actions(&mut self, constant_actions: Vec<AssignedConstantAction>) {
        self.constant_actions = constant_actions;
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
