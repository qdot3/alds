use std::{collections::VecDeque, marker::PhantomData};

use super::CSR;

pub struct Dijkstra<W> {
    source: usize,
    distance: Vec<Option<usize>>,
    parent: Vec<Option<usize>>,

    weight_type: PhantomData<W>,
}

impl Dijkstra<()> {
    /// # Panics
    ///
    /// Panics if `source` is out of bounds.
    pub fn new(csr: &CSR<()>, source: usize) -> Self {
        let mut distance = vec![None; csr.num_nodes()];
        distance[source] = Some(0);
        let mut parent = vec![None; csr.num_nodes()];

        // 01DP
        let mut next = VecDeque::with_capacity(csr.num_nodes());
        next.push_back(source);
        while let Some(source) = next.pop_front() {
            for e in csr.edges(source) {
                // if dist[tar].is_some(), then dist[tar] <= dist[src] + 1.
                if distance[e.target()].is_none() {
                    distance[e.target()] = distance[e.source()].map(|d| d + 1);
                    parent[e.target()] = Some(e.source());

                    next.push_back(e.target());
                }
            }
        }

        Self {
            source,
            distance,
            parent,
            weight_type: PhantomData::<()>,
        }
    }
}

impl<W> Dijkstra<W> {
    pub const fn source(&self) -> usize {
        self.source
    }

    pub fn distance(&self, target: usize) -> Option<usize> {
        self.distance.get(target).and_then(|&d| d)
    }

    pub fn shortest_path(&self, target: usize) -> Option<Vec<usize>> {
        if let Some(d) = self.distance(target) {
            let mut path = Vec::with_capacity(d + 1);
            path.push(target);
            for i in 0..d {
                path.push(self.parent[path[i]].unwrap());
            }
            path.reverse();

            Some(path)
        } else {
            None
        }
    }
}
