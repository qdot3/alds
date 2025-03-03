use std::cell::Cell;

/// Union Find with Potential
///
/// # Performance note
///
/// | [new](UnionFindWithPotential::new) | [find](UnionFindWithPotential::find)/[size](UnionFindWithPotential::size)/[same](UnionFindWithPotential::same)/[unite](UnionFindWithPotential::unite)/[potential](UnionFindWithPotential::potential) |
/// |------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | *O*(*N*)                           | *O*(α(*N*)), amortized                                                                                                                                                                               |
///
/// * α(*N*) is the functional inverse of Ackermann's function which diverges very slowly.
#[derive(Debug, Clone)]
pub struct UnionFindWithPotential<P: Group> {
    node: Vec<Cell<Node<P>>>,
}

impl<P: Group> UnionFindWithPotential<P> {
    const MAX_SIZE: usize = i32::MAX as usize + 1; // 2^31

    pub fn new(size: usize) -> Self {
        assert!(size <= Self::MAX_SIZE);

        Self {
            node: vec![Cell::new(Node::new()); size],
        }
    }

    pub fn find(&self, i: usize) -> usize {
        // path compression
        if let Some(p) = self.node[i].get().get_parent() {
            let ri = self.find(p);
            // P(i) = L @ P(parent) = L @ L' @ P(root)
            self.node[i].set(Node {
                par_or_size: ri as i32,
                potential: (self.node[i].get().potential())
                    .binary_operation(self.node[p].get().potential()),
            });

            return ri;
        }

        i
    }

    pub fn same(&self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    pub fn size(&self, i: usize) -> usize {
        self.node[self.find(i)]
            .get()
            .get_size()
            .expect("node should be a root node")
    }

    /// get P_ij of `P(i) = P_ij @ P(j)`
    pub fn potential(&self, i: usize, j: usize) -> Option<P> {
        if !self.same(i, j) {
            return None;
        }

        // the parent is the root due to path compression.
        // P(i) = Pi @ P(root), P(j) = Pj @ P(root) => P(i) = Pi @ inv(Pj) @ P(j)
        // => P_ij = Pi @ inv(Pj)
        Some(
            (self.node[i].get().potential())
                .binary_operation(self.node[j].get().potential().inverse()),
        )
    }

    /// Sets P(i) = P_ij @ P(j) if there is no contradiction
    pub fn unite(&mut self, i: usize, j: usize, mut potential_ij: P) -> Result<bool, ()> {
        if let Some(p_ij) = self.potential(i, j) {
            if potential_ij == p_ij {
                return Ok(false);
            } else {
                return Err(());
            }
        }

        // very ugly!
        {
            let mut ri = self.find(i);
            let mut rj = self.find(j);
            let Self { node } = self;

            if node[ri].get().get_size().unwrap() < node[rj].get().get_size().unwrap() {
                std::mem::swap(&mut ri, &mut rj);
                potential_ij = potential_ij.inverse();
            }

            node[ri].get_mut().par_or_size += node[rj].get().par_or_size;
            // P(i) = Pi @ P(ri), P(j) = Pj @ P(rj), P(i) = P_ij @ P(j)
            // => P(rj) = inv(Pj) @ inv(P_ij) @ Pi * P(ri)
            let new_rj = Node {
                par_or_size: ri as i32,
                potential: (node[j].get().potential().inverse())
                    .binary_operation(potential_ij.inverse())
                    .binary_operation(node[i].get().potential()),
            };
            // enforce mutability
            node[rj] = Cell::new(new_rj)
        }

        Ok(true)
    }
}

#[derive(Debug, Clone, Copy)]
struct Node<P: Group> {
    par_or_size: i32,
    /// P(self) = P * P(parent)
    potential: P,
}

impl<P: Group> Node<P> {
    fn new() -> Self {
        Self {
            par_or_size: -1,
            potential: P::identity(),
        }
    }

    fn get_parent(&self) -> Option<usize> {
        if self.par_or_size.is_negative() {
            None
        } else {
            Some(self.par_or_size as usize)
        }
    }

    fn get_size(&self) -> Option<usize> {
        if self.par_or_size.is_negative() {
            Some(-self.par_or_size as usize)
        } else {
            None
        }
    }

    fn potential(&self) -> P {
        self.potential
    }
}

pub trait Group: Copy + PartialEq + Eq {
    fn identity() -> Self;
    /// associative binary operation
    fn binary_operation(&self, rhs: Self) -> Self;
    fn inverse(&self) -> Self;
}
