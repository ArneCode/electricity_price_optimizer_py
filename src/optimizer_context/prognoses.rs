use std::ops::Add;

use crate::{
    optimizer_context::action::constant::AssignedConstantAction,
    time::{STEPS_PER_DAY, Time},
};

// const MINUTES_PER_DAY: usize = 24 * 60;

#[derive(Clone)]
pub struct Prognoses<T: Clone> {
    data: [T; STEPS_PER_DAY as usize],
}

impl<T: Clone> Prognoses<T> {
    pub fn new(data: [T; STEPS_PER_DAY as usize]) -> Self {
        Self { data }
    }

    pub fn get(&self, time: Time) -> Option<&T> {
        self.data.get(time.to_timestep() as usize)
    }

    pub fn set(&mut self, time: Time, value: T) {
        if time.to_timestep() < STEPS_PER_DAY {
            self.data[time.to_timestep() as usize] = value;
        }
    }

    pub fn get_data(&self) -> &[T; STEPS_PER_DAY as usize] {
        &self.data
    }
}

impl<T: From<i32> + Add<T, Output = T> + Clone> Prognoses<T> {
    pub fn add_constant_action(&mut self, action: &AssignedConstantAction) {
        let start = action.get_start_time().to_timestep() as usize;
        let end = action.get_end_time().to_timestep() as usize;
        let consumption = action.get_action().get_consumption();

        for t in start..end {
            self.data[t] = self.data[t].clone() + T::from(consumption);
        }
    }
}
