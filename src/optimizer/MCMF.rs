use std::collections::VecDeque;

const INF: i64 = 1_i64 << 60;

struct Edge {
    to: usize,
    f: i64,
    cost: i64,
}

pub struct MinCostFlow {
    edges: Vec<Edge>,
    adj: Vec<Vec<usize>>,
    pref: Vec<Option<usize>>,
    con: Vec<usize>,
    dist: Vec<i64>,
    s: u32,
    t: u32,
    pub maxflow: i64,
    pub mincost: i64,
}

impl MinCostFlow {
    pub fn new(n: usize, source: u32, target: u32) -> Self {
        Self {
            edges: Vec::new(),
            adj: vec![Vec::new(); n],
            pref: Vec::new(),
            con: Vec::new(),
            dist: Vec::new(),
            s: source,
            t: target,
            maxflow: 0,
            mincost: 0,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, capacity: i64, cost: i64) {
        self.adj[u].push(self.edges.len());
        self.edges.push(Edge { to: v, f: capacity, cost });
        self.adj[v].push(self.edges.len());
        self.edges.push(Edge { to: u, f: 0, cost: -cost });
    }

    fn spfa(&mut self) -> bool {
        let n = self.adj.len();
        self.pref = vec![None; n];
        self.dist = vec![INF; n];
        let mut inqueue = vec![false; n];
        let mut queue = VecDeque::new();

        self.dist[self.s] = 0;
        self.pref[self.s] = Some(self.s);
        inqueue[self.s] = true;
        queue.push_back(self.s);

        while let Some(cur) = queue.pop_front() {
            inqueue[cur] = false;
            for i in 0..self.adj[cur].len() {
                let id = self.adj[cur][i];
                let to = self.edges[id].to;
                if self.edges[id].f > 0
                    && self.dist[to] > self.dist[cur] + self.edges[id].cost
                {
                    self.dist[to] = self.dist[cur] + self.edges[id].cost;
                    self.pref[to] = Some(cur);
                    self.con[to] = id;
                    if !inqueue[to] {
                        inqueue[to] = true;
                        queue.push_back(to);
                    }
                }
            }
        }

        self.pref[self.t].is_some()
    }

    fn extend(&mut self) {
        let mut w = INF;
        let mut u = self.t;
        while self.pref[u] != Some(u) {
            w = w.min(self.edges[self.con[u]].f);
            u = self.pref[u].unwrap();
        }

        self.maxflow += w;
        self.mincost += self.dist[self.t] * w;

        u = self.t;
        while self.pref[u] != Some(u) {
            let edge_id = self.con[u];
            self.edges[edge_id].f -= w;
            self.edges[edge_id ^ 1].f += w;
            u = self.pref[u].unwrap();
        }
    }

    pub fn mincostflow(&mut self, flow: i32) {
        let n = self.adj.len();
        self.con = vec![0; n];
        self.maxflow = 0;
        self.mincost = 0;
        while self.spfa() {
            self.extend();
        }
    }
}
