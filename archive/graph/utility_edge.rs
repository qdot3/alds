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

    pub fn is_self_loop(&self) -> bool {
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
