use std::collections::VecDeque;

const INF: i64 = 1_i64 << 60;

#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    rev: usize, // Store index of reverse edge for O(1) access
    f: i64,
    cost: i64,
}

pub struct MinCostFlow {
    graph: Vec<Vec<Edge>>,
    dist: Vec<i64>,
    pref: Vec<Option<(usize, usize)>>, // (node, edge_index)
    inqueue: Vec<bool>,
    queue: VecDeque<usize>, // Reused queue buffer
    s: usize,
    t: usize,
    pub maxflow: i64,
    pub mincost: i64,
}

impl MinCostFlow {
    pub fn new(n: usize, source: usize, target: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            dist: vec![INF; n],
            pref: vec![None; n],
            inqueue: vec![false; n],
            queue: VecDeque::with_capacity(n), // Pre-allocate queue
            s: source,
            t: target,
            maxflow: 0,
            mincost: 0,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, capacity: i64, cost: i64) {
        let fwd_idx = self.graph[u].len();
        let rev_idx = self.graph[v].len();

        self.graph[u].push(Edge {
            to: v,
            rev: rev_idx,
            f: capacity,
            cost,
        });
        self.graph[v].push(Edge {
            to: u,
            rev: fwd_idx,
            f: 0,
            cost: -cost,
        });
    }

    fn spfa(&mut self) -> bool {
        // RESET buffers without reallocation
        self.dist.fill(INF);
        self.pref.fill(None);
        // inqueue is guaranteed to be all false at end of loop, no need to fill

        self.dist[self.s] = 0;
        self.queue.push_back(self.s);
        self.inqueue[self.s] = true;

        while let Some(u) = self.queue.pop_front() {
            self.inqueue[u] = false;

            for i in 0..self.graph[u].len() {
                let e = self.graph[u][i]; // Copy is cheap (struct is small)

                if e.f > 0 && self.dist[e.to] > self.dist[u] + e.cost {
                    self.dist[e.to] = self.dist[u] + e.cost;
                    self.pref[e.to] = Some((u, i));

                    if !self.inqueue[e.to] {
                        // SLF Optimization: If new dist is smaller than front of queue, push front
                        if let Some(&front) = self.queue.front() {
                            if self.dist[e.to] < self.dist[front] {
                                self.queue.push_front(e.to);
                            } else {
                                self.queue.push_back(e.to);
                            }
                        } else {
                            self.queue.push_back(e.to);
                        }
                        self.inqueue[e.to] = true;
                    }
                }
            }
        }

        self.dist[self.t] != INF
    }

    pub fn mincostflow(&mut self) -> (i64, i64) {
        self.maxflow = 0;
        self.mincost = 0;

        while self.spfa() {
            let mut flow = INF;
            let mut cur = self.t;

            // Backtrack to find bottleneck
            while cur != self.s {
                let (prev, idx) = self.pref[cur].unwrap();
                flow = flow.min(self.graph[prev][idx].f);
                cur = prev;
            }

            // Apply flow
            cur = self.t;
            while cur != self.s {
                let (prev, idx) = self.pref[cur].unwrap();
                let rev_idx = self.graph[prev][idx].rev;

                self.graph[prev][idx].f -= flow;
                self.graph[cur][rev_idx].f += flow;

                self.mincost += flow * self.graph[prev][idx].cost;
                cur = prev;
            }
            self.maxflow += flow;
        }
        (self.maxflow, self.mincost)
    }
}
