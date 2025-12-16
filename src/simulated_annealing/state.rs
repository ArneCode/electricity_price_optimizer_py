use crate::{
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
    constant_actions: Vec<AssignedConstantAction>,

    context: OptimizerContext,
}

impl State {
    pub fn new(context: OptimizerContext) -> Self {
        let constant_actions = context
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
        Self {
            constant_actions,
            context,
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
    pub fn get_context(&self) -> &OptimizerContext {
        &self.context
    }

    pub fn to_fixed_context(&self) -> OptimizerContext {
        let mut new_context = self.context.clone();
        for action in &self.constant_actions {
            new_context.add_constant_action_to_consumption(action);
        }
        new_context
    }
}
