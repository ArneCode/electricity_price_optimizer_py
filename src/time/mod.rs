use std::{
    fmt::Display,
    ops::{Add, Sub},
};

const MINUTES_PER_TIMESTEP: u32 = 15;

const MINUTES_PER_DAY: u32 = 60 * 24;
pub const STEPS_PER_DAY: u32 = MINUTES_PER_DAY / MINUTES_PER_TIMESTEP;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    minutes: u32,
}

impl Time {
    pub fn new(hours: u32, minutes: u32) -> Self {
        Self {
            minutes: hours * 60 + minutes,
        }
    }

    pub fn to_timestep(&self) -> u32 {
        self.minutes / MINUTES_PER_TIMESTEP
    }

    pub fn from_timestep(timestep: u32) -> Self {
        Self {
            minutes: timestep * MINUTES_PER_TIMESTEP,
        }
    }

    pub fn get_minutes(&self) -> u32 {
        self.minutes
    }
}

impl Add<Time> for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time {
            minutes: self.minutes + other.minutes,
        }
    }
}

impl Sub<Time> for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time {
            minutes: self.minutes - other.minutes,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
