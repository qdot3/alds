use std::{cell::Cell, marker::PhantomData};

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
    /// let mut uf = UnionFind::new(100);
    /// assert_eq!(uf.leader(0), 0);
    ///
    /// uf.merge(0, 1);
    /// assert_eq!(uf.leader(0), uf.leader(1));
    /// assert_ne!(uf.leader(0), uf.leader(2));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if given node is unknown.
    ///
    /// # Time complexity
    ///
    /// *a*(*n*), where *a* is the inverse of Ackermann function
    pub fn leader(&self, a: usize) -> usize {
        if self.par_or_size[a].get().is_negative() {
            return a;
        }
        // path compression
        let ra = self.leader(self.par_or_size[a].get() as usize);
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
    /// let mut uf = UnionFind::new(100);
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
        self.leader(a) == self.leader(b)
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
        self.par_or_size[self.leader(a)].get().abs() as usize
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
        //* use `&mut self` since belongings of nodes may change.*//

        let mut ra = self.leader(a);
        let mut rb = self.leader(b);

        if ra == rb {
            return false;
        }

        // union by size
        if self.par_or_size[ra] > self.par_or_size[rb] {
            std::mem::swap(&mut ra, &mut rb)
        }
        //* this method changes belongings of nodes.*//
        self.par_or_size[ra] = Cell::new(self.par_or_size[ra].take() + self.par_or_size[rb].get());
        self.par_or_size[rb] = Cell::new(ra as i32);

        true
    }

    /// Returns iterator of groups.
    ///
    /// # Example
    ///
    /// ```
    /// use alds::union_find::UnionFind;
    ///
    /// let mut uf = UnionFind::new(100);
    /// for i in (2..100).step_by(2) {
    ///     uf.merge(0, i);
    ///     uf.merge(1, i + 1);
    /// }
    /// assert_eq!(uf.size(0), 50);
    /// assert_eq!(uf.size(1), 50);
    ///
    /// for group in uf.groups() {
    ///     assert!(!group.is_empty());
    ///
    ///     let parity = group[0] % 2;
    ///     assert!(group.into_iter().all(|i| i % 2 == parity));
    /// }
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(*n* *a*(*n*)), where *a* is the inverse of Ackermann function
    pub fn groups<'a>(&self) -> Groups<'a> {
        let n = self.par_or_size.len();
        let mut group_id = vec![usize::MAX; n];
        let mut size = Vec::with_capacity(n);
        for (gi, i) in (0..n)
            .filter(|&i| self.par_or_size[i].get().is_negative())
            .enumerate()
        {
            group_id[i] = gi;
            size.push(self.par_or_size[i].get().abs() as usize);
        }

        let mut groups = Vec::from_iter(size.into_iter().map(|n| Vec::with_capacity(n)));
        for i in 0..n {
            groups[group_id[self.leader(i)]].push(i);
        }

        Groups {
            groups,
            _marker: PhantomData,
        }
    }
}

pub struct Groups<'a> {
    groups: Vec<Vec<usize>>,

    _marker: PhantomData<&'a UnionFind>,
}

impl<'a> Iterator for Groups<'a> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.groups.pop()
    }
}
