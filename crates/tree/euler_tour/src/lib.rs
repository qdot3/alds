pub struct EulerTour {
    first: Box<[usize]>,
    last: Box<[usize]>,
    expanded: Box<[usize]>,
}

impl EulerTour {
    pub fn new(parents: Vec<usize>, root: usize) -> Self {
        const NULL: usize = usize::MAX;
        let mut first = vec![NULL; parents.len() + 1].into_boxed_slice();
        let mut last = first.clone();

        let mut stack = Vec::with_capacity(parents.len());
        stack.push(root);
        let mut expanded = Vec::with_capacity(parents.len() * 2 + 1);
        let mut time = 0;
        let mut children = vec![Vec::new()];
        for (i, p) in parents.into_iter().enumerate() {
            children[p].push(i)
        }
        while let Some(i) = stack.pop() {
            expanded.push(i);
            if first[i] == NULL {
                first[i] = time;
            }
            last[i] = time;

            stack.extend(
                std::mem::take(&mut children[i])
                    .into_iter()
                    .map(|c| [i, c])
                    .flatten(),
            );

            time += 1;
        }

        Self {
            first,
            last,
            expanded: expanded.into_boxed_slice(),
        }
    }

    pub fn expanded(&self) -> &[usize] {
        &self.expanded
    }
}
