use std::ops::Add;

use crate::optimizer_context::action::constant::AssignedConstantAction;

const MINUTES_PER_DAY: usize = 24 * 60;

#[derive(Clone)]
pub struct Prognoses<T: Clone> {
    data: [T; MINUTES_PER_DAY],
}

impl<T: Clone> Prognoses<T> {
    pub fn new(data: [T; MINUTES_PER_DAY]) -> Self {
        Self { data }
    }

    pub fn get(&self, minute: usize) -> Option<&T> {
        self.data.get(minute)
    }

    pub fn set(&mut self, minute: usize, value: T) {
        if minute < MINUTES_PER_DAY {
            self.data[minute] = value;
        }
    }

    pub fn get_data(&self) -> &[T; MINUTES_PER_DAY] {
        &self.data
    }
}

impl<T: From<i32> + Add<T, Output = T> + Clone> Prognoses<T> {
    pub fn add_constant_action(&mut self, action: &AssignedConstantAction) {
        let start = action.get_start_time() as usize;
        let end = action.get_end_time() as usize;
        let consumption = action.get_action().get_consumption();

        for t in start..end {
            self.data[t] = self.data[t].clone() + T::from(consumption);
        }
    }
}
