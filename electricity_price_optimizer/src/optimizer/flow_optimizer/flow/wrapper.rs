use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{optimizer::flow_optimizer::flow::MinCostFlow, time::Time};

#[derive(Clone)]
pub struct FlowWrapper {
    pub inner: MinCostFlow,
    node_map: HashMap<FlowNode, usize>,
}

impl FlowWrapper {
    pub fn new() -> Self {
        let inner = MinCostFlow::new();
        let node_map = HashMap::from([
            (FlowNode::Source, inner.get_source()),
            (FlowNode::Sink, inner.get_sink()),
        ]);
        Self { inner, node_map }
    }

    fn node(&mut self, key: FlowNode) -> usize {
        if let Some(&id) = self.node_map.get(&key) {
            id
        } else {
            let id = self.inner.new_node();
            self.node_map.insert(key, id);
            id
        }
    }

    pub fn add_edge(&mut self, u: FlowNode, v: FlowNode, cap: i64, cost: i64) -> usize {
        let u_id = self.node(u);
        let v_id = self.node(v);
        self.inner.add_edge(u_id, v_id, cap, cost)
    }

    pub fn new_node(&mut self) -> usize {
        self.inner.new_node()
    }

    pub fn mincostflow(&mut self) -> (i64, i64) {
        self.inner.mincostflow()
    }
}

impl Default for FlowWrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for FlowWrapper {
    type Target = MinCostFlow;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for FlowWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum FlowNode {
    Wire(Time),           // timestep
    Action(usize),        // action id
    Battery(usize, Time), // battery id, timestep
    Source,
    Sink,
    Network,
    Generator,
}
