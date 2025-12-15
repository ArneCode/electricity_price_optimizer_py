use std::{
    cmp::{Reverse},
    collections::{BinaryHeap, VecDeque},
    io::{self, Read, Write},
};

const INF: i64 = 1_i64 << 60;

#[derive(Clone)]
struct Edge {
    to: usize,
    f: i64,
    cost: i64,
}

struct MinCostFlow {
    edges: Vec<Edge>,
    adj: Vec<Vec<usize>>,
    pref: Vec<usize>,
    con: Vec<usize>,
    dist: Vec<i64>,
    pi: Vec<i64>,
    s: usize,
    t: usize,
    maxflow: i64,
    mincost: i64,
}

impl MinCostFlow {
    fn new(n: usize) -> Self {
        Self {
            edges: Vec::new(),
            adj: vec![Vec::new(); n],
            pref: Vec::new(),
            con: Vec::new(),
            dist: Vec::new(),
            pi: Vec::new(),
            s: 0,
            t: 0,
            maxflow: 0,
            mincost: 0,
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: i64, cost: i64) {
        self.adj[u].push(self.edges.len());
        self.edges.push(Edge { to: v, f: cap, cost });
        self.adj[v].push(self.edges.len());
        self.edges.push(Edge { to: u, f: 0, cost: -cost });
    }

    fn spfa(&mut self) -> bool {
        let n = self.adj.len();
        self.pref = vec![usize::MAX; n];
        self.dist = vec![INF; n];
        let mut inq = vec![false; n];
        let mut q = VecDeque::new();

        self.dist[self.s] = 0;
        self.pref[self.s] = self.s;
        q.push_back(self.s);
        inq[self.s] = true;

        while let Some(u) = q.pop_front() {
            inq[u] = false;
            for &id in &self.adj[u] {
                let e = &self.edges[id];
                if e.f > 0 && self.dist[e.to] > self.dist[u] + e.cost {
                    self.dist[e.to] = self.dist[u] + e.cost;
                    self.pref[e.to] = u;
                    self.con[e.to] = id;
                    if !inq[e.to] {
                        inq[e.to] = true;
                        q.push_back(e.to);
                    }
                }
            }
        }
        self.pref[self.t] != usize::MAX
    }

    fn dijkstra(&mut self) -> bool {
        let n = self.adj.len();
        // reset predecessor, distance
        self.pref = vec![usize::MAX; n];
        self.dist = vec![INF; n];
        
        // min‐heap of (dist, node)
        let mut heap = BinaryHeap::new();

        // start at source
        self.dist[self.s] = 0;
        self.pref[self.s] = self.s;
        heap.push(Reverse((0, self.s)));

        while let Some(Reverse((d, u))) = heap.pop() {
            // stale entry?
            if d != self.dist[u] {
                continue;
            }
            // relax all residual edges out of u
            for &id in &self.adj[u] {
                let e = &self.edges[id];
                if e.f > 0 {
                    let v = e.to;
                    let nd = d + (e.cost - self.pi[v] + self.pi[u]);
                    if nd < self.dist[v] {
                        self.dist[v] = nd;
                        self.pref[v] = u;
                        self.con[v] = id;
                        heap.push(Reverse((nd, v)));
                    }
                }
            }
        }

        if self.pref[self.t] == usize::MAX {
            return false;
        }

        for i in 0..self.dist.len() {
            if self.pi[i] != INF {
                self.dist[i] -= self.pi[self.s] - self.pi[i];
            }
        }

        true
    }

    fn extend(&mut self) {
        let mut w = INF;
        let mut u = self.t;
        while self.pref[u] != u {
            let id = self.con[u];
            w = w.min(self.edges[id].f);
            u = self.pref[u];
        }

        self.maxflow += w;
        self.mincost += self.dist[self.t] * w;

        let mut u = self.t;
        while self.pref[u] != u {
            let id = self.con[u];
            self.edges[id].f -= w;
            self.edges[id ^ 1].f += w;
            u = self.pref[u];
        }

        for i in 0..self.pi.len() {
          self.pi[i] = self.dist[i];
        }
    }

    fn mincostflow(&mut self, s: usize, t: usize) {
        self.s = s;
        self.t = t;
        let n = self.adj.len();
        self.con = vec![0; n];
        self.pi = vec![0; n];
        self.maxflow = 0;
        self.mincost = 0;
        while self.dijkstra() {
            self.extend();
        }
    }
}
