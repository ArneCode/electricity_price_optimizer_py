use std::rc::Rc;

use crate::time::Time;

pub struct ConstantAction {
    pub start_from: Time,
    pub end_before: Time,
    pub duration: Time,
    pub consumption: i32,
    id: u32,
}
impl ConstantAction {
    pub fn new(
        start_from: Time,
        end_before: Time,
        duration: Time,
        consumption: i32,
        id: u32,
    ) -> Self {
        assert!(
            start_from + duration <= end_before,
            "Invalid constant action time bounds"
        );
        Self {
            start_from,
            end_before,
            duration,
            consumption,
            id,
        }
    }
    pub fn get_start_from(&self) -> Time {
        self.start_from
    }
    pub fn get_end_before(&self) -> Time {
        self.end_before
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_consumption(&self) -> i32 {
        self.consumption
    }
}

pub struct AssignedConstantAction {
    action: Rc<ConstantAction>,
    start_time: Time,
}
impl AssignedConstantAction {
    pub fn new(action: Rc<ConstantAction>, start_time: Time) -> Self {
        assert!(
            start_time >= action.start_from && start_time + action.duration <= action.end_before,
            "Start time is out of bounds for the constant action"
        );
        Self { action, start_time }
    }

    pub fn get_start_time(&self) -> Time {
        self.start_time
    }

    pub fn get_start_time_mut(&mut self) -> &mut Time {
        &mut self.start_time
    }

    pub fn get_action(&self) -> &Rc<ConstantAction> {
        &self.action
    }

    pub fn get_end_time(&self) -> Time {
        self.start_time + self.action.duration
    }
}
