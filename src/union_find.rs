use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct UnionFind {
    par_or_size: Vec<Cell<i32>>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            par_or_size: vec![Cell::new(-1); size],
        }
    }

    pub fn find_root(&self, a: usize) -> usize {
        if self.par_or_size[a].get().is_negative() {
            return a;
        }
        // path compression
        let ra = self.find_root(self.par_or_size[a].get() as usize);
        self.par_or_size[a].set(ra as i32);

        ra
    }

    pub fn is_same(&self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    pub fn size(&self, a: usize) -> usize {
        self.par_or_size[self.find_root(a)].get().abs() as usize
    }

    pub fn merge(&self, a: usize, b: usize) -> bool {
        let mut ra = self.find_root(a);
        let mut rb = self.find_root(b);

        if ra == rb {
            return false;
        }

        // union by size
        if self.par_or_size[ra] > self.par_or_size[rb] {
            std::mem::swap(&mut ra, &mut rb)
        }
        self.par_or_size[ra].set(self.par_or_size[ra].take() + self.par_or_size[rb].get());
        self.par_or_size[rb].set(ra as i32);

        true
    }

    pub fn group(&self) -> Vec<Vec<usize>> {
        let n = self.par_or_size.len();
        let mut group_id = vec![usize::MAX; n];
        let mut group = Vec::with_capacity(n);
        for (gi, i) in (0..n)
            .filter(|&i| self.par_or_size[i].get().is_negative())
            .enumerate()
        {
            group_id[i] = gi;
            group.push(Vec::with_capacity(self.size(i)));
        }
        for i in 0..n {
            group[group_id[self.find_root(i)]].push(i)
        }

        group
    }
}
