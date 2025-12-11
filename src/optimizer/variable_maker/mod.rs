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
pub const FORK_FROM_SOURCE: ItemId = 4;

pub const WIRE: ItemId = -42;

pub type PersistentVariableIndex = (ItemId, Timestamp);

// Third parameter stands whether node stands of incoming, meaning all incoming edges must visit incoming = true, and exit from incoming = false
pub type PersistentVariableWithCapacityIndex = (ItemId, Timestamp, bool);

pub struct VariableMaker { 
    variable_count: u32,
    
    persistent_variable_indices: BiMap<PersistentVariableIndex, u32>,
    persistent_variable_with_capacity_index: BiMap<PersistentVariableWithCapacityIndex, u32>
}
impl VariableMaker {
    pub fn new(context: &OptimizerContext) -> Self {
        let mut variable_count: u32 = (FORK_FROM_SOURCE + 1) as u32;
        let mut persistent: BiMap<PersistentVariableIndex, u32> = BiMap::new();
        let mut persistent_with_capacity: BiMap<PersistentVariableWithCapacityIndex, u32> = BiMap::new();
        
        for var_action in context.get_variable_actions().iter() {
            let item_id = var_action.get_id() as ItemId;
            for t in 0..MINUTES_PER_DAY {
                persistent_with_capacity.insert((item_id as ItemId, t as Timestamp, false), variable_count);
                variable_count += 1;
                persistent_with_capacity.insert((item_id as ItemId, t as Timestamp, true), variable_count);
                variable_count += 1;
            }
        }

        for battery in context.get_batteries().iter() {
            let item_id = battery.get_id() as ItemId;
            for t in 0..MINUTES_PER_DAY {
                persistent_with_capacity.insert((item_id as ItemId, t as Timestamp, false), variable_count);
                variable_count += 1;
                persistent_with_capacity.insert((item_id as ItemId, t as Timestamp, true), variable_count);
                variable_count += 1;
            }
        }

        for t in 0..MINUTES_PER_DAY {
            persistent.insert((WIRE, t as Timestamp), variable_count);
            variable_count += 1;
        }

        Self {
            variable_count,
            persistent_variable_indices: persistent,
            persistent_variable_with_capacity_index: persistent_with_capacity,
        }
    }

    pub fn get_action_variable_index(&self, item_id: ItemId, timestamp: Timestamp, incoming: bool) -> Option<u32> {
        self.persistent_variable_with_capacity_index.get_by_a(&(item_id, timestamp, incoming)).cloned()
    }
    pub fn get_battery_variable_index(&self, item_id: ItemId, timestamp: Timestamp, incoming: bool) -> Option<u32> {
        self.persistent_variable_with_capacity_index.get_by_a(&(item_id, timestamp, incoming)).cloned()
    }

    pub fn get_wire_index(&self, timestamp: Timestamp) -> Option<u32> {
        self.persistent_variable_indices.get_by_a(&(WIRE, timestamp)).cloned()
    }

    pub fn get_variable_count(&self) -> u32 {
        return self.variable_count;
    }
}
