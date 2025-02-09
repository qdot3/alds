use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct UnionFind {
    par_or_size: Vec<Cell<i32>>,
}

impl UnionFind {
    /// Creates union find tree with *n* nodes.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::union_find::UnionFind;
    ///
    /// let mut uf = UnionFind::new(1_000);
    /// uf.merge(0, 2);
    /// uf.merge(0, 1);
    ///
    /// assert!(uf.is_same(1, 2));
    /// assert!(!uf.is_same(1, 3));
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(*n*)
    pub fn new(size: usize) -> Self {
        Self {
            par_or_size: vec![Cell::new(-1); size],
        }
    }

    /// Returns the leader of the group that given node belongs.
    ///
    /// Leaders may change when two groups are merged.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::union_find::UnionFind;
    ///
    /// let mut uf = UnionFind::new(1_000);
    /// assert_eq!(uf.find_root(0), 0);
    ///
    /// uf.merge(0, 1);
    /// assert_eq!(uf.find_root(0), uf.find_root(1));
    /// assert_ne!(uf.find_root(0), uf.find_root(2));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if given node is unknown.
    ///
    /// # Time complexity
    ///
    /// *a*(*n*), where *a* is the inverse of Ackermann function
    pub fn find_root(&self, a: usize) -> usize {
        if self.par_or_size[a].get().is_negative() {
            return a;
        }
        // path compression
        let ra = self.find_root(self.par_or_size[a].get() as usize);
        self.par_or_size[a].set(ra as i32);

        ra
    }

    /// Check if given two node is in the same group.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::union_find::UnionFind;
    ///
    /// let mut uf = UnionFind::new(1_000);
    /// assert!(!uf.is_same(0, 1));
    /// uf.merge(0, 1);
    /// assert!(uf.is_same(0, 1));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if given node is unknown.
    ///
    /// # Time complexity
    ///
    /// *a*(*n*), where *a* is the inverse of Ackermann function
    pub fn is_same(&self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }

    /// Returns the size of the group that given node belongs.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::union_find::UnionFind;
    ///
    /// let mut uf = UnionFind::new(100);
    /// assert!((0..100).all(|i| uf.size(i) == 1));
    ///
    /// uf.merge(0, 1);
    /// uf.merge(1, 2);
    /// uf.merge(2, 3);
    /// assert!((0..4).all(|i| uf.size(i) == 4));
    /// assert!((4..100).all(|i| uf.size(i) == 1));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if given node is unknown.
    ///
    /// # Time complexity
    ///
    /// *a*(*n*), where *a* is the inverse of Ackermann function
    pub fn size(&self, a: usize) -> usize {
        self.par_or_size[self.find_root(a)].get().abs() as usize
    }

    /// Merge two groups that given nodes belong respectively.
    ///
    /// If they have been already in the same group, do nothing and returns `false`.
    /// Otherwise, returns `true`.
    ///
    /// # Example
    /// ```
    /// use alds::union_find::UnionFind;
    ///
    /// let mut uf = UnionFind::new(100);
    /// 
    /// assert!(uf.merge(0, 1));
    /// assert!(!uf.merge(0, 1));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if given node is unknown.
    ///
    /// # Time complexity
    ///
    /// *a*(*n*), where *a* is the inverse of Ackermann function
    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        //* use `&mut self` since this method may change belongings of nodes.*//

        let mut ra = self.find_root(a);
        let mut rb = self.find_root(b);

        if ra == rb {
            return false;
        }

        // union by size
        if self.par_or_size[ra] > self.par_or_size[rb] {
            std::mem::swap(&mut ra, &mut rb)
        }
        //* this method may change belongings of nodes.*//
        self.par_or_size[ra] = Cell::new(self.par_or_size[ra].take() + self.par_or_size[rb].get());
        self.par_or_size[rb] = Cell::new(ra as i32);

        true
    }

    /// Returns groups.
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