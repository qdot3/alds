#[derive(Debug, Clone)]
pub struct LCA {
    depth: Box<[usize]>,
    dfs_preorder: Box<[usize]>,
    ancestor_table: Box<[usize]>,
    len: usize,
}

impl LCA {
    /// # Panics
    ///
    /// Panics if given edges does NOT represent a tree.
    pub fn from_edges(edges: Vec<(usize, usize)>, root: usize) -> Self {
        // dfsで深さをきめる。
        // lca_many()のために、行きがけ順を求めておく
        // 親ノードでダブリング。テーブルのサイズは n * max_depth.ilog2()

        let n = edges.len() + 1;
        let mut edge = vec![Vec::new(); n];
        for (u, v) in edges {
            edge[u].push(v);
            edge[v].push(u);
        }

        let mut dfs_stack = Vec::with_capacity(n);
        dfs_stack.push(root);
        const NULL: usize = !0;
        let mut depth = vec![NULL; n].into_boxed_slice();
        let mut max_depth = 0;
        let mut dfs_preorder = vec![NULL; n].into_boxed_slice();
        let mut counter = 0;
        let mut parent = vec![NULL; n];
        parent[root] = root;
        while let Some(&i) = dfs_stack.last() {
            if depth[i] == NULL {
                // NULL + 1 = 0 for the root node
                depth[i] = depth[parent[i]].wrapping_add(1);
                max_depth = max_depth.max(depth[i]);

                for j in std::mem::take(&mut edge[i]) {
                    if depth[j] == NULL {
                        parent[j] = i;
                        dfs_stack.push(j)
                    }
                }
            } else {
                dfs_stack.pop();

                dfs_preorder[i] = counter;
                counter += 1;
            }
        }

        let mut ancestor_table = Vec::with_capacity(n * max_depth.ilog2() as usize);
        for _ in 0..=max_depth.ilog2() {
            ancestor_table.extend(parent.iter().copied());
            parent = Vec::from_iter(parent.iter().map(|&i| parent[i]))
        }

        Self {
            depth,
            dfs_preorder,
            ancestor_table: ancestor_table.into_boxed_slice(),
            len: n,
        }
    }

    /// Returns the lowest common ancestor of given pair and distance between them.
    pub fn lca(&self, mut i: usize, mut j: usize) -> (usize, usize) {
        // ノードの深さをそろえる
        // ダブリングで祖先をたどる。祖先が一致したら、その１つ前にセット。
        // 繰り返すと２つの異なるノードの親が一致するようになる。それがLCA

        if i == j {
            return (i, 0);
        }

        let Self {
            depth,
            dfs_preorder: _,
            ancestor_table,
            len,
        } = self;
        let d = depth[i] + depth[j];

        // step 1
        if depth[i] < depth[j] {
            std::mem::swap(&mut i, &mut j);
        }
        let mut diff = depth[i] - depth[j];
        while diff > 0 {
            let k = diff.trailing_zeros() as usize;
            diff ^= 1 << k;
            i = ancestor_table[len * k + i];
        }

        if i == j {
            return (i, d - depth[i] * 2);
        }

        // step 2
        for k in (0..ancestor_table.len() / len).rev() {
            if ancestor_table[len * k + i] != ancestor_table[len * k + j] {
                i = ancestor_table[len * k + i];
                j = ancestor_table[len * k + j];
            }
        }

        let lca = ancestor_table[i];
        let dist = d - 2 * depth[lca];
        (lca, dist)
    }

    /// Returns the lca of given nodes minimum length of path which connects all of them.
    pub fn lca_many(&self, mut node_list: Vec<usize>) -> Option<(usize, usize)> {
        // ３つ以上のノードのLCAとすべての頂点を結ぶ最短パスの長さを求める
        // dfs preorderでノードをソートして、順にLCAを計算
        node_list.sort_unstable_by_key(|&i| self.dfs_preorder[i]);
        node_list.dedup();

        if node_list.len() > 2 {
            let (mut lca, mut len) = self.lca(node_list[0], node_list[1]);
            for pair in node_list.windows(2).skip(1) {
                let (lca1, _) = self.lca(pair[0], pair[1]);
                let (lca2, len2) = self.lca(lca, pair[1]);
                if self.depth[lca1] < self.depth[lca2] {
                    len += self.depth[lca1] - self.depth[pair[1]]
                } else {
                    len += len2;
                    lca = lca2
                }
            }

            Some((lca, len))
        } else if node_list.len() == 2 {
            Some(self.lca(node_list[0], node_list[1]))
        } else {
            None
        }
    }
}
