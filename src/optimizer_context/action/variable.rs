use std::rc::Rc;

pub struct VariableAction {
    pub start: i32,
    pub end: i32,
    pub total_consumption: i32,
    pub max_consumption: i32,
    id: u32,
}

impl VariableAction {
    pub fn new(
        start: i32,
        end: i32,
        total_consumption: i32,
        max_consumption: i32,
        id: u32,
    ) -> Self {
        assert!(
            start < end,
            "Invalid variable action time bounds: start must be less than end"
        );
        Self {
            start,
            end,
            total_consumption,
            max_consumption,
            id,
        }
    }
    pub fn get_start(&self) -> i32 {
        self.start
    }
    pub fn get_end(&self) -> i32 {
        self.end
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_total_consumption(&self) -> i32 {
        self.total_consumption
    }
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
