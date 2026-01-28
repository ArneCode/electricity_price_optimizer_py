use std::collections::HashMap;

use crate::optimizer_context::{
    action::{constant::AssignedConstantAction, variable::AssignedVariableAction},
    battery::AssignedBattery,
    prognoses::Prognoses,
};

#[derive(Debug, Clone)]
pub struct Schedule {
    pub constant_actions: HashMap<u32, AssignedConstantAction>,
    pub variable_actions: HashMap<u32, AssignedVariableAction>,
    pub batteries: HashMap<u32, AssignedBattery>,
    pub network_consumption: Prognoses<i32>,
}

impl Schedule {
    pub fn new(
        constant_actions: HashMap<u32, AssignedConstantAction>,
        variable_actions: HashMap<u32, AssignedVariableAction>,
        batteries: HashMap<u32, AssignedBattery>,
        network_consumption: Prognoses<i32>,
    ) -> Self {
        Self {
            constant_actions,
            variable_actions,
            batteries,
            network_consumption,
        }
    }

    pub fn set_constant_actions(&mut self, actions: HashMap<u32, AssignedConstantAction>) {
        self.constant_actions = actions;
    }

    pub fn get_variable_action(&self, id: u32) -> Option<&AssignedVariableAction> {
        self.variable_actions.get(&id)
    }

    pub fn get_constant_action(&self, id: u32) -> Option<&AssignedConstantAction> {
        self.constant_actions.get(&id)
    }

    pub fn get_battery(&self, id: u32) -> Option<&AssignedBattery> {
        self.batteries.get(&id)
    }
}
