use crate::{
    action::{constant::ConstantAction, variable::VariableAction},
    prognoses::{ElectricityPrognoses, PricePrognoses},
};

pub struct EnvironmentData {
    electricity_price: PricePrognoses,
    generated_electricity: ElectricityPrognoses,
    beyond_control_consumption: ElectricityPrognoses,

    constant_actions: Vec<ConstantAction>,
    variable_actions: Vec<VariableAction>,
}
