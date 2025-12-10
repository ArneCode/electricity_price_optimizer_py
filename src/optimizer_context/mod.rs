pub mod action;
pub mod battery;
pub mod prognoses;

use crate::optimizer_context::{
    action::{constant::ConstantAction, variable::VariableAction},
    battery::Battery,
    prognoses::{ElectricityPrognoses, PricePrognoses},
};

pub struct OptimizerContext {
    electricity_price: PricePrognoses,
    generated_electricity: ElectricityPrognoses,
    beyond_control_consumption: ElectricityPrognoses,
    batteries: Vec<Battery>,

    constant_actions: Vec<ConstantAction>,
    variable_actions: Vec<VariableAction>,
}
