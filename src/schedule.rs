use crate::action::constant_action::ConstantAction;
use crate::action::variable_action::VariableAction;

struct Schedule {
    pub constant_actions: Vec<(i32, ConstantAction)>,
    pub variable_actions: Vec<(i32, VariableAction)>,
}
