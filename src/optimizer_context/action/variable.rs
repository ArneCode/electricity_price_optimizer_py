use std::rc::Rc;

pub struct VariableAction {
    pub start: i32,
    pub end: i32,
    pub total_consumption: i32,
    pub max_consumption: i32,
    id: i32,
}
pub struct AssignedVariableAction {
    action: Rc<VariableAction>,
    consumption: Vec<u32>,
}

impl AssignedVariableAction {
    pub fn new(action: Rc<VariableAction>, consumption: Vec<u32>) -> Self {
        assert_eq!(
            consumption.len() as i32,
            action.end - action.start,
            "Consumption list length does not match action duration"
        );
        Self {
            action,
            consumption,
        }
    }
}
