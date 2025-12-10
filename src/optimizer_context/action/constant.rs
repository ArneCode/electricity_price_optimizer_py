use std::rc::Rc;

pub struct ConstantAction {
    pub start_from: i32,
    pub end_before: i32,
    pub duration: i32,
    pub consumption: i32,
    id: i32
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
}
