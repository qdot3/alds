use super::Edge;

#[derive(Debug, Clone)]
pub struct CSR<W> {
    edges: Vec<Edge<W>>,
    start: Vec<usize>,
}

impl<W> CSR<W> {
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn num_nodes(&self) -> usize {
        unimplemented!()
    }

    pub fn edges(&self, source: usize) -> &[Edge<W>] {
        if let Some(&end) = self.start.get(source + 1) {
            &self.edges[self.start[source]..end]
        } else {
            &self.edges[self.start[source]..]
        }
    }
}

#[derive(Debug, Clone)]
pub struct CSRBuilder<W> {
    edges: Vec<Edge<W>>,
    num_out: Vec<usize>,
    num_in: Vec<usize>,
}

impl<W> CSRBuilder<W> {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            num_out: Vec::new(),
            num_in: Vec::new(),
        }
    }
}

impl<W> From<Vec<Edge<W>>> for CSRBuilder<W> {
    fn from(value: Vec<Edge<W>>) -> Self {
        if let Some(max) = value
            .iter()
            .map(|e| [e.source(), e.target()])
            .flatten()
            .max()
        {
            let mut num_out = vec![0; max + 1];
            let mut num_in = vec![0; max + 1];
            for e in &value {
                num_out[e.source()] += 1;
                num_in[e.target()] += 1;
            }

            Self {
                edges: value,
                num_out,
                num_in,
            }
        } else {
            Self::new()
        }
    }
}
