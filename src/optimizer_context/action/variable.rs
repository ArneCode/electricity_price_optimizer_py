use std::rc::Rc;

use crate::time::Time;

pub struct VariableAction {
    pub start: Time,
    pub end: Time,
    pub total_consumption: i32,
    pub max_consumption: i32,
    id: u32,
}

impl VariableAction {
    pub fn new(
        start: Time,
        end: Time,
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
    pub fn get_start(&self) -> Time {
        self.start
    }
    pub fn get_end(&self) -> Time {
        self.end
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_total_consumption(&self) -> i32 {
        self.total_consumption
    }
    pub fn get_max_consumption(&self) -> i32 {
        self.max_consumption
    }
}

pub struct AssignedVariableAction {
    action: Rc<VariableAction>,
    consumption: Vec<u32>,
}

impl AssignedVariableAction {
    pub fn new(action: Rc<VariableAction>, consumption: Vec<u32>) -> Self {
        assert_eq!(
            consumption.len() as u32,
            action.end.to_timestep() - action.start.to_timestep(),
            "Consumption list length does not match action duration"
        );
        Self {
            action,
            consumption,
        }
    }
}
