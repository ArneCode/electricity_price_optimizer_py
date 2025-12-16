use std::collections::HashMap;

pub struct FlowWrapper {
    pub inner: MinCostFlow,
    node_map: HashMap<(usize, usize), usize>,
}

impl FlowWrapper {
    pub fn new() -> Self {
        Self {
            inner: MinCostFlow::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn source(&self) -> usize {
        self.inner.get_source()
    }

    pub fn sink(&self) -> usize {
        self.inner.get_sink()
    }

    pub fn node(&mut self, key: (usize, usize)) -> usize {
        if let Some(&id) = self.node_map.get(&key) {
            id
        } else {
            let id = self.inner.new_node();
            self.node_map.insert(key, id);
            id
        }
    }

    pub fn add_edge<U: IntoNode, V: IntoNode>(
        &mut self,
        u: U,
        v: V,
        cap: i64,
        cost: i64,
    ) -> usize {
        let u_id = u.into_node(self);
        let v_id = v.into_node(self);
        self.inner.add_edge(u_id, v_id, cap, cost)
    }

    pub fn mincostflow(&mut self) -> (i64, i64) {
        self.inner.mincostflow()
    }
}

pub trait IntoNode {
    fn into_node(self, w: &mut FlowWrapper) -> usize;
}

impl IntoNode for usize {
    fn into_node(self, _w: &mut FlowWrapper) -> usize {
        self
    }
}

impl IntoNode for (usize, usize) {
    fn into_node(self, w: &mut FlowWrapper) -> usize {
        w.node(self)
    }
}


/*
Usage:

let mut flow = FlowWrapper::new();
let s = flow.source();
let t = flow.sink();

flow.add_edge(s, (0, 0), 10, 0);
flow.add_edge((0, 0), (1, 0), 5, 1);
flow.add_edge((1, 0), t, 10, 0);

let (cost, max_flow) = flow.mincostflow();
*/