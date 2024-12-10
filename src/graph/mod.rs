mod dfs;

#[derive(Debug, Clone)]
pub struct Edge<W> {
    source: usize,
    target: usize,
    weight: W,
}

impl<W> Edge<W> {
    pub fn new(source: usize, target: usize, weight: W) -> Self {
        Self {
            source,
            target,
            weight,
        }
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.source, &mut self.target);
    }

    pub fn is_loop(&self) -> bool {
        self.source == self.target
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn target(&self) -> usize {
        self.target
    }

    pub fn weight(&self) -> &W {
        &self.weight
    }

    pub fn weight_mut(&mut self) -> &mut W {
        &mut self.weight
    }
}

impl From<(usize, usize)> for Edge<()> {
    fn from(value: (usize, usize)) -> Self {
        Edge {
            source: value.0,
            target: value.1,
            weight: (),
        }
    }
}

impl<W> From<(usize, usize, W)> for Edge<W> {
    fn from(value: (usize, usize, W)) -> Self {
        Edge {
            source: value.0,
            target: value.1,
            weight: value.2,
        }
    }
}

#[derive(Debug, Clone)]
pub(self) struct EdgeCSR<W> {
    edges: Vec<Edge<W>>,
    csr_start: Vec<usize>,
    edge_id: Vec<usize>,
}

impl<W> EdgeCSR<W> {
    pub(self) fn out_edges(&self, source: usize) -> impl Iterator<Item = &Edge<W>> + '_ {
        let start = self.csr_start[source];
        let end = self
            .csr_start
            .get(source + 1)
            .copied()
            .unwrap_or(self.edge_id.len());

        (start..end).map(|i| &self.edges[self.edge_id[i]])
    }

    pub(self) fn num_vertexes(&self) -> usize {
        self.csr_start.len()
    }

    pub(self) fn num_edges(&self) -> usize {
        self.edge_id.len()
    }
}

impl<W> From<Vec<Edge<W>>> for EdgeCSR<W> {
    // Calculate in $O(|V| + |E|)$ time/space complexity.
    fn from(edges: Vec<Edge<W>>) -> Self {
        if let Some(n) = edges.iter().map(|e| [e.source, e.target]).flatten().max() {
            let mut csr_start = vec![0; n + 1];
            for e in &edges {
                csr_start[e.source] += 1;
            }
            let mut csr_start = Vec::from_iter(csr_start.into_iter().scan(0, |state, r| {
                *state += r;
                Some(*state)
            }));
            let mut edge_id = vec![0; edges.len()];
            for (i, e) in edges.iter().enumerate() {
                csr_start[e.source] -= 1;
                edge_id[csr_start[e.source]] = i;
            }

            Self {
                edges,
                csr_start,
                edge_id,
            }
        } else {
            Self {
                edges: vec![],
                csr_start: vec![],
                edge_id: vec![],
            }
        }
    }
}

impl<W> Extend<Edge<W>> for EdgeCSR<W> {
    fn extend<T: IntoIterator<Item = Edge<W>>>(&mut self, iter: T) {
        self.edges.extend(iter);
        *self = Self::from(std::mem::take(&mut self.edges));
    }
}

#[derive(Debug, Clone)]
pub struct Graph<W> {
    edge_csr: EdgeCSR<W>,
}

impl<W> Extend<Edge<W>> for Graph<W> {
    fn extend<T: IntoIterator<Item = Edge<W>>>(&mut self, iter: T) {
        self.edge_csr.extend(iter);
    }
}

impl<W, V> FromIterator<V> for Graph<W>
where
    V: Into<Edge<W>>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let edge_csr = EdgeCSR::from(Vec::from_iter(iter.into_iter().map(|v| v.into())));

        Self { edge_csr }
    }
}
