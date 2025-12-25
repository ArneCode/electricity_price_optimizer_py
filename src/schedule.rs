use crate::optimizer_context::{
    action::{constant::AssignedConstantAction, variable::AssignedVariableAction},
    battery::AssignedBattery,
    prognoses::Prognoses,
};

#[derive(Debug, Clone)]
pub struct Schedule {
    pub constant_actions: Vec<AssignedConstantAction>,
    pub variable_actions: Vec<AssignedVariableAction>,
    pub batteries: Vec<AssignedBattery>,
    pub network_consumption: Prognoses<i32>,
}

impl Schedule {
    pub fn new(
        constant_actions: Vec<AssignedConstantAction>,
        variable_actions: Vec<AssignedVariableAction>,
        batteries: Vec<AssignedBattery>,
        network_consumption: Prognoses<i32>,
    ) -> Self {
        Self {
            constant_actions,
            variable_actions,
            batteries,
            network_consumption,
        }
    }

    pub fn set_constant_actions(&mut self, actions: Vec<AssignedConstantAction>) {
        self.constant_actions = actions;
    }
}
