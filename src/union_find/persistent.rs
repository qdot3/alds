use std::iter::Once;

#[derive(Debug, Clone)]
pub struct PersistentUnionFind {
    par_or_size: Vec<Once<i32>>,
}

impl PersistentUnionFind {
    pub fn new() -> Self {
        todo!()
    }
}
