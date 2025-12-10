use crate::optimizer_context::action::{constant::ConstantAction, variable::AssignedVariableAction};

pub struct Schedule {
    pub constant_actions: Vec<(i32, ConstantAction)>,
    pub variable_actions: Vec<AssignedVariableAction>,
}
