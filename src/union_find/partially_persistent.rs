/// Partially Persistent Union Find Tree.
///
/// # Partially persistent data structure
///
/// You can query any version but can update only the latest version.
///
/// # Performance note
///
/// | method                                         | time complexity |
/// |------------------------------------------------|-----------------|
/// | [`new`](PartiallyPersistentUnionFind::new)     | *O*(*N*)        |
/// | [`find`](PartiallyPersistentUnionFind::find)   | *O*(log *N*)    |
/// | [`size`](PartiallyPersistentUnionFind::size)   | *O*(log *N*)    |
/// | [`same`](PartiallyPersistentUnionFind::same)   | *O*(log *N*)    |
/// | [`unite`](PartiallyPersistentUnionFind::unite) | *O*(log *N*)    |
/// | others                                         | *O*(1)          |
#[derive(Debug, Clone)]
pub struct PartiallyPersistentUnionFind {
    node: Vec<Node>,
    now: u32,
}

impl PartiallyPersistentUnionFind {
    const MAX_CAPACITY: usize = i32::MAX as usize + 1; // or 2^31

    pub fn new(size: usize) -> Self {
        assert!(size <= Self::MAX_CAPACITY);

        Self {
            node: vec![Node::new(); size],
            now: 0,
        }
    }

    pub const fn current_time(&self) -> u32 {
        self.now
    }

    pub fn find(&self, i: usize, time: u32) -> usize {
        if self.node[i].is_root(time) {
            return i;
        }
        self.find(self.node[i].get_parent().unwrap(), time)
    }

    pub fn same(&self, i: usize, j: usize, time: u32) -> bool {
        self.find(i, time) == self.find(j, time)
    }

    pub fn size(&self, i: usize, time: u32) -> usize {
        self.node[self.find(i, time)].size(time) as usize
    }

    pub fn unite(&mut self, i: usize, j: usize) -> bool {
        let mut ri = self.find(i, self.now);
        let mut rj = self.find(j, self.now);

        if ri == rj {
            return false;
        }

        // very ugly!
        {
            let Self { node, now } = self;

            assert_eq!(node[ri].time_updated, Node::TIME_INFINITY);
            assert_eq!(node[rj].time_updated, Node::TIME_INFINITY);

            if node[ri].par_or_rank > node[rj].par_or_rank {
                std::mem::swap(&mut ri, &mut rj);
            } else if node[ri].par_or_rank == node[rj].par_or_rank {
                // keep `size_history` small
                if node[ri].size_history.len() > node[rj].size_history.len() {
                    std::mem::swap(&mut ri, &mut rj);
                }
                node[ri].par_or_rank -= 1;
            }

            *now += 1;

            let size = node[ri].size(*now) + node[rj].size(*now);
            node[ri].size_history.push((*now, size));
            node[rj].par_or_rank = ri as i32;
            node[rj].time_updated = *now;
        }

        true
    }
}

#[derive(Debug, Clone)]
struct Node {
    /// non-negative integer means parent index, negative integer means rank (height of the tree).
    par_or_rank: i32,
    time_updated: u32,
    /// Vec<(time, size)>
    size_history: Vec<(u32, u32)>,
}

impl Node {
    /// Since the number of times any two nodes are united is at most `i32::MAX + 1`,
    /// `u32::MAX` can be treated as infinity as long as the time advances by 1
    /// when and only when `unite` operation succeeds.
    const TIME_INFINITY: u32 = u32::MAX;

    #[inline]
    const fn new() -> Self {
        Self {
            par_or_rank: -1,
            time_updated: Self::TIME_INFINITY,
            // initialize with not `vec![(0, 1)]` but `Vec::new()` to save space
            size_history: Vec::new(),
        }
    }

    #[inline]
    const fn is_root(&self, time: u32) -> bool {
        self.time_updated > time
    }

    #[inline]
    const fn get_parent(&self) -> Option<usize> {
        if self.par_or_rank.is_negative() {
            None
        } else {
            Some(self.par_or_rank as usize)
        }
    }

    /// *O* (log *N*)
    #[inline]
    #[rustfmt::skip]
    fn size(&self, time: u32) -> u32 {
        let i = self.size_history.partition_point(|&(t, _)| t <= time);

        if i == 0 { 1 } else { self.size_history[i - 1].1 }
    }
}
