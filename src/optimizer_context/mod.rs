pub mod action;
pub mod battery;
pub mod prognoses;

use std::rc::Rc;

use crate::optimizer_context::{
    action::{constant::{AssignedConstantAction, ConstantAction}, variable::VariableAction},
    battery::Battery,
    prognoses::Prognoses,
};

#[derive(Clone)]
pub struct OptimizerContext {
    electricity_price: Rc<Prognoses<i32>>,
    generated_electricity: Rc<Prognoses<i32>>,
    beyond_control_consumption: Prognoses<i32>,

    batteries: Rc<Vec<Battery>>,

    constant_actions: Vec<Rc<ConstantAction>>,
    variable_actions: Vec<Rc<VariableAction>>,
}
impl OptimizerContext {
    pub fn new(
        electricity_price: Prognoses<i32>,
        generated_electricity: Prognoses<i32>,
        beyond_control_consumption: Prognoses<i32>,
        batteries: Vec<Battery>,
        constant_actions: Vec<Rc<ConstantAction>>,
        variable_actions: Vec<Rc<VariableAction>>,
    ) -> Self {
        Self {
            electricity_price: Rc::new(electricity_price),
            generated_electricity: Rc::new(generated_electricity),
            beyond_control_consumption,
            batteries: Rc::new(batteries),
            constant_actions,
            variable_actions,
        }
    }

    pub fn get_constant_actions(&self) -> &Vec<Rc<ConstantAction>> {
        &self.constant_actions
    }
    pub fn get_variable_actions(&self) -> &Vec<Rc<VariableAction>> {
        &self.variable_actions
    }
    pub fn get_batteries(&self) -> &Vec<Battery> {
        &self.batteries
    }

    pub fn add_constant_action_to_consumption(&mut self, action: &AssignedConstantAction) {
        self.beyond_control_consumption.add_constant_action(action);
    }

    pub fn get_electricity_price(&self) -> &Rc<Prognoses<i32>> {
        &self.electricity_price
    }

    pub fn get_generated_electricity(&self) -> &Rc<Prognoses<i32>> {
        &self.generated_electricity
    }

    pub fn get_beyond_control_consumption(&self) -> &Prognoses<i32> {
        &self.beyond_control_consumption
    }
}
