use std::rc::Rc;

pub struct ConstantAction {
    pub start_from: i32,
    pub end_before: i32,
    pub duration: i32,
    pub consumption: i32,
    id: u32
}
impl ConstantAction {
    pub fn new(start_from: i32, end_before: i32, duration: i32, consumption: i32, id: u32) -> Self {
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
    pub fn get_start_from(&self) -> i32 {
        self.start_from
    }
    pub fn get_end_before(&self) -> i32 {
        self.end_before
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
}

pub struct AssignedConstantAction {
    action: Rc<ConstantAction>,
    start_time: i32,
}
impl AssignedConstantAction {
    pub fn new(action: Rc<ConstantAction>, start_time: i32) -> Self {
        assert!(
            start_time >= action.start_from && start_time + action.duration <= action.end_before,
            "Start time is out of bounds for the constant action"
        );
        Self {
            action,
            start_time,
        }
    }

    pub fn get_start_time(&self) -> i32 {
        self.start_time
    }

    pub fn get_start_time_mut(&mut self) -> &mut i32 {
        &mut self.start_time
    }

    pub fn get_action(&self) -> &Rc<ConstantAction> {
        &self.action
    }
}
