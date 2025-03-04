use std::cell::Cell;

use num_traits::Signed;

#[derive(Debug, Clone)]
pub struct WeightedUnionFind<W: Copy> {
    par_or_size: Vec<Cell<i32>>,
    diff_weight: Vec<Cell<W>>,
}

impl<W: Copy + Signed> WeightedUnionFind<W> {
    pub fn new(size: usize) -> Self {
        Self {
            par_or_size: vec![Cell::new(1); size],
            diff_weight: vec![Cell::new(W::zero()); size],
        }
    }

    pub fn find_root(&self, a: usize) -> usize {
        if self.par_or_size[a].get().is_negative() {
            return a;
        }
        // path compression
        let ra = self.find_root(self.par_or_size[a].get() as usize);
        self.diff_weight[a].set(
            self.diff_weight[a].get() + self.diff_weight[self.par_or_size[a].get() as usize].get(),
        );
        self.par_or_size[a].set(ra as i32);

        ra
    }

    pub fn is_same(&self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    pub fn size(&self, a: usize) -> usize {
        (-self.par_or_size[self.find_root(a)].get()) as usize
    }

    pub fn weight(&self, a: usize) -> W {
        self.find_root(a);
        //* Now, `a` is a child of the root. */
        self.diff_weight[a].get()
    }

    /// `weight[b] = weight[a] + w`
    pub fn union(&mut self, a: usize, b: usize, mut w: W) -> Result<bool, ()> {
        let mut ra = self.find_root(a);
        let mut rb = self.find_root(b);
        w = w + self.diff_weight[a].get() - self.diff_weight[b].get();

        if ra == rb {
            if self.diff_weight[ra].get() == self.diff_weight[rb].get() + w {
                return Ok(false);
            } else {
                return Err(());
            }
        }

        // union by size
        if self.par_or_size[ra] > self.par_or_size[rb] {
            std::mem::swap(&mut ra, &mut rb);
            w = -w;
        }
        //* enforce `&mut self` since this method may change belongings of nodes.*//
        self.par_or_size[ra] = Cell::new(self.par_or_size[a].get() + self.par_or_size[rb].get());
        self.diff_weight[rb] = Cell::new(w);
        self.par_or_size[rb] = Cell::new(ra as i32);

        Ok(true)
    }
}
