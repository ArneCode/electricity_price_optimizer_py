use std::collections::HashMap;
use crate::optimizer_context::OptimizerContext;
use crate::variable_maker::DoubleSidedHash::BiMap;

pub type ItemId = u64;
pub type Timestamp = u64;

pub type PersistentVariableIndex = (ItemId, Timestamp);
pub type NonpersistentVariableIndex = ItemId;

pub struct VariableMaker { 
    context: OptimizerContext,
    
    persistent_variable_indices: BiMap<PersistentVariableIndex, usize>,
    nonpersistent_variable_indices: BiMap<NonpersistentVariableIndex, usize>,
}
impl VariableMaker {
    pub fn new(context: OptimizerContext) -> Self {
        let mut persistent = BiMap::new();
        let mut nonpersistent = BiMap::new();

        // To do: Populate the BiMaps based on the context.
        // This is required for MCMF as it needs integer indices for variables.

        Self {
            context,
            persistent_variable_indices: persistent,
            nonpersistent_variable_indices: nonpersistent,
        }
    }
}