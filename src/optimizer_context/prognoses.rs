use std::ops::Add;

use crate::{
    optimizer_context::action::constant::AssignedConstantAction,
    time::{STEPS_PER_DAY, Time},
};

// const MINUTES_PER_DAY: usize = 24 * 60;

/// Holds prognoses data for each timestep in a day.
/// For example, electricity prices, generated electricity, or beyond control consumption.
#[derive(Clone)]
pub struct Prognoses<T: Clone> {
    /// Data for each timestep in a day.
    data: [T; STEPS_PER_DAY as usize],
}

impl<T: Clone> Prognoses<T> {
    pub fn new(data: [T; STEPS_PER_DAY as usize]) -> Self {
        Self { data }
    }

    pub fn get(&self, time: Time) -> Option<&T> {
        self.data.get(time.to_timestep() as usize)
    }

    /// Sets the value at the given time.
    ///
    /// # Arguments
    /// * `time` - The time at which to set the value.
    /// * `value` - The value to set.
    /// # Notes
    /// If the time is out of bounds, the function does nothing.
    pub fn set(&mut self, time: Time, value: T) {
        if time.to_timestep() < STEPS_PER_DAY {
            self.data[time.to_timestep() as usize] = value;
        }
    }

    /// Returns a reference to the internal data array.
    pub fn get_data(&self) -> &[T; STEPS_PER_DAY as usize] {
        &self.data
    }
}

impl<T: From<i32> + Add<T, Output = T> + Clone> Prognoses<T> {
    /// Adds the consumption of a constant action to the prognoses data.
    /// Used to update consumption prognoses when scheduling constant actions.
    ///
    /// # Arguments
    /// * `action` - The assigned constant action to add.
    pub fn add_constant_action(&mut self, action: &AssignedConstantAction) {
        let start = action.get_start_time().to_timestep() as usize;
        let end = action.get_end_time().to_timestep() as usize;
        let consumption = action.get_action().get_consumption();

        for t in start..end {
            self.data[t] = self.data[t].clone() + T::from(consumption);
        }
    }
}
