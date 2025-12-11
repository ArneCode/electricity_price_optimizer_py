pub mod DoubleSidedHash;

use std::collections::HashMap;

use crate::optimizer_context::OptimizerContext;
use crate::optimizer::variable_maker::DoubleSidedHash::BiMap;
use crate::optimizer_context::action::variable;

const MINUTES_PER_DAY: u32 = 60 * 24;

pub type ItemId = i32;
pub type Timestamp = u32;

pub const SOURCE: ItemId = 0;
pub const SINK: ItemId = 1;
pub const NETWORK: ItemId = 2;
pub const GENERATOR: ItemId = 3;

pub const SYSTEM_FLOW: ItemId = -42;


pub type PersistentVariableIndex = (ItemId, Timestamp);

pub struct VariableMaker { 
    variable_count: u32,
    
    persistent_variable_indices: BiMap<PersistentVariableIndex, usize>,
}
impl VariableMaker {
    pub fn new(context: &OptimizerContext) -> Self {
        let mut variable_count = 4;
        let mut persistent = BiMap::new();
        
        for var_action in context.get_variable_actions().iter() {
            let item_id = var_action.get_id() as ItemId;
            for t in 0..MINUTES_PER_DAY {
                persistent.insert((item_id as ItemId, t as Timestamp), variable_count);
                variable_count += 1;
            }
        }

        for battery in context.get_batteries().iter() {
            let item_id = battery.get_id() as ItemId;
            for t in 0..MINUTES_PER_DAY {
                persistent.insert((item_id as ItemId, t as Timestamp), variable_count);
                variable_count += 1;
            }
        }

        for t in 0..MINUTES_PER_DAY {
            persistent.insert((SYSTEM_FLOW, t as Timestamp), variable_count);
            variable_count += 1;
        }

        Self {
            variable_count,
            persistent_variable_indices: persistent,
        }
    }

    pub fn get_action_variable_index(&self, item_id: ItemId, timestamp: Timestamp) -> Option<usize> {
        self.persistent_variable_indices.get_by_a(&(item_id, timestamp)).cloned()
    }
    pub fn get_constant_action_variable_index(&self, item_id: ItemId, timestamp: Timestamp) -> Option<usize> {
        self.persistent_variable_indices.get_by_a(&(item_id, timestamp)).cloned()
    }
    pub fn get_battery_variable_index(&self, item_id: ItemId, timestamp: Timestamp) -> Option<usize> {
        self.persistent_variable_indices.get_by_a(&(item_id, timestamp)).cloned()
    }

    pub fn get_system_flow_index(&self, timestamp: Timestamp) {
        self.persistent_variable_indices.get_by_a(&(SYSTEM_FLOW, timestamp)).cloned();
    }

    pub fn get_variable_count(&self) -> u32 {
        return self.variable_count;
    }
}
