use super::{Edge, Graph};

impl<W> Graph<W> {
    pub fn dfs_with(&self, source: usize, mut op: impl FnMut(&Edge<W>) -> ()) {
        let n = self.edge_csr.num_vertexes();
        let mut is_visited = vec![false; n];
        let mut stack = Vec::new();
        stack.push(source);
        while let Some(i) = stack.pop() {
            if is_visited[i] {
                continue;
            }
            is_visited[i] = true;

            for e in self.edge_csr.out_edges(i) {
                if !is_visited[e.target] {
                    op(e);
                    stack.push(e.target);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::graph::Graph;

    #[test]
    fn find_depth() {
        let g = Graph::from_iter(vec![(0, 1), (0, 2), (1, 3), (1, 4)]);
        let mut depth = vec![0; 5];
        g.dfs_with(0, |e| depth[e.target] = depth[e.source] + 1);

        assert_eq!(depth, vec![0, 1, 1, 2, 2])
    }
}
