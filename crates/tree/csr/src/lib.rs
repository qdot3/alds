/// Compressed sparse row for sparse graph.
pub struct CSR<N, E> {
    node_list: Vec<N>,
    edge_list: Vec<(usize, usize, E)>,
}

impl<N, E> CSR<N, E> {
    pub fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            node_list: Vec::with_capacity(node_capacity),
            edge_list: Vec::with_capacity(edge_capacity),
        }
    }

    pub fn push_node(&mut self, weight: N) -> usize {
        self.node_list.push(weight);
        self.node_list.len() - 1
    }

    /// Appends a directed edge.
    pub fn push_edge(&mut self, src: usize, tar: usize, weight: E) {
        self.edge_list.push((src, tar, weight))
    }

    
}
