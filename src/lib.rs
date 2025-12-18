use crate::{optimizer_context::OptimizerContext, schedule::Schedule};

pub mod optimizer;
pub mod optimizer_context;
mod schedule;
pub mod simulated_annealing;
pub mod time;
mod helper;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn optimize(data: OptimizerContext) -> Schedule {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
