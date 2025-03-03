use std::cmp::Ordering;

use stash::{Tag, UniqueStash};

/// Implicit eager pairing heap.
pub struct PairingHeap2<P, I> {
    data: UniqueStash<Node<P, I>>,
    root: Option<Tag>,
}

impl<P: Ord, I> PairingHeap2<P, I> {
    pub fn contains(&self, tag: Tag) -> bool {
        self.data.get(tag).is_some()
    }

    /// Appends a new item to the pairing heap.
    ///
    /// Use [`Self::raise_priority`] for stored item.
    pub fn push(&mut self, item: I, priority: P) -> Tag {
        let tag = self.data.put(Node::new(item, priority));

        if let Some(root_tag) = self.root.replace(tag) {
            self.root = Some(self.meld(root_tag, tag))
        }

        tag
    }

    pub fn peek(&self) -> Option<&Entry<P, I>> {
        self.root.map(|tag| self.data[tag].ref_entry())
    }

    pub fn get(&self, tag: Tag) -> Option<&Entry<P, I>> {
        self.data.get(tag).map(|node| node.ref_entry())
    }

    pub fn pop(&mut self) -> Option<Entry<P, I>> {
        if let Some(root_tag) = self.root.take() {
            self.root = self.detach_and_orchestrate_children(root_tag);

            let prev_root = self.data.take(root_tag).expect("root_tag is valid");
            Some(prev_root.into_entry())
        } else {
            None
        }
    }

    // pub fn remove(&mut self, tag: Tag) -> Option<Entry<P, I>> {
    //     if self.root.is_some_and(|root_tag| root_tag == tag) {
    //         self.pop()
    //     } else if self.data.get(tag).is_some() {
    //         // parent
    //         //  └ this
    //         //     ├ (child)  <- detach if exists
    //         //     └ (sibling)
    //         if let Some(child) = self.detach_and_orchestrate_children(tag) {
    //             self.root = Some(self.meld(self.root.unwrap(), child))
    //         }

    //         // parent                                   parent
    //         //  └ this                              =>   └ (sibling)
    //         //     └ (sibling) <- detach if exists      this <- root
    //         assert!(self.detach_node(tag).is_some());

    //         Some(self.data.take(tag).unwrap().into_entry())
    //     } else {
    //         None
    //     }
    // }

    pub fn update_priority(&mut self, tag: Tag, new_priority: P) -> bool {
        if !self.contains(tag) {
            return false;
        }

        // do nothing
        if self.data[tag].ref_entry().priority == new_priority {
            return true;
        }

        match self.data[tag].update_priority(new_priority) {
            Ordering::Less => todo!(),
            Ordering::Equal => (),
            Ordering::Greater => {
                if let Some(parent) = self.data[tag].take_parent() {
                    if self.data[parent].priority_cmp(&self.data[tag]).is_lt() {}
                }
            }
        }
        todo!()
    }

    /// Merges two root nodes, then returns `Tag` of the new root (`root_1` or `root_2`).
    ///
    /// # Panics
    ///
    /// Panic if given tags have expired or may panic if they are tied with non-root nodes.
    fn meld(&mut self, mut root_1: Tag, mut root_2: Tag) -> Tag {
        if self.data[root_1].priority_cmp(&self.data[root_2]).is_lt() {
            std::mem::swap(&mut root_1, &mut root_2);
        }

        // `root_1` has priority over `root_2`.
        assert!(self.data[root_2].replace_parent(root_1).is_none());
        if let Some(child_1) = self.data[root_1].replace_child(root_2) {
            assert!(self.data[root_2].replace_sibling(child_1).is_none())
        }

        root_1
    }

    /// Detaches children, orchestrates them by pairing, then returns the tag of their root.
    ///
    /// ```text
    /// (root)?                 (root)?
    ///  └ this                  └ this
    ///     ┝ (child)?       =>     ┝ (child)
    ///     |  ┝ (child)?           |  ┝ (child)?  <───┐ orchestrate siblings
    ///     |  └ (sibling)?         |  └ [no sibling] ─┘
    ///     └ (sibling)?            └ (sibling)?
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if given tag has expired
    fn detach_and_orchestrate_children(&mut self, tag: Tag) -> Option<Tag> {
        if let Some(mut new_root) = self.data[tag].take_child() {
            assert!(self.data[new_root].take_parent().is_some());

            // iterate over children and orchestrate them
            let mut next = self.data[new_root].take_sibling();
            while let Some(first) = next.take() {
                // detach from the parent for consistency
                assert!(self.data[first].take_parent().is_some());

                new_root = if let Some(second) = self.data[first].take_sibling() {
                    assert!(self.data[second].take_parent().is_some());
                    next = self.data[second].take_sibling();

                    let pair = self.meld(first, second);
                    self.meld(new_root, pair)
                } else {
                    next = None;

                    self.meld(new_root, first)
                };
            }

            Some(new_root)
        } else {
            None
        }
    }

    /// Detaches node from the tree.
    /// See this:
    ///
    /// ```text
    /// (1) parent              parent          | (2) [no parent]           (child)?  <- new root
    ///      └ this          =>  └ (sibling)?   |      └ this  <- root    >  ┝ (child of child)?
    ///         ┝ (child)?      this            |         ┝ (child)?         └ [no sibling]
    ///         └ (sibling)?     └ (child)?     |         └ [no sibling]    this
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if given tag is expired.
    fn detach_node(&mut self, tag: Tag) {
        if let Some(parent) = self.data[tag].take_parent() {
            // attaches the sibling to the parent
            if let Some(sibling) = self.data[tag].take_sibling() {
                if self.data[parent].has_child(tag) {
                    self.data[parent].replace_child(sibling);
                } else {
                    assert_eq!(self.data[parent].replace_sibling(sibling), Some(tag))
                }
            }
        } else {
            // orchestrates the children and update the root.
            self.root = todo!()
        }
    }
}

#[derive(Debug)]
pub struct Entry<P, I> {
    pub priority: P,
    pub item: I,
}

struct Node<P, I> {
    entry: Entry<P, I>,

    parent: Option<Tag>,
    child: Option<Tag>,
    sibling: Option<Tag>,
}

impl<P: Ord, I> Node<P, I> {
    const fn new(item: I, priority: P) -> Self {
        Self {
            entry: Entry { priority, item },
            parent: None,
            child: None,
            sibling: None,
        }
    }

    const fn is_root(&self) -> bool {
        self.parent.is_none() && self.sibling.is_none()
    }

    fn priority_cmp(&self, other: &Self) -> Ordering {
        self.entry.priority.cmp(&other.entry.priority)
    }

    fn update_priority(&mut self, new_priority: P) -> Ordering {
        let order = self.entry.priority.cmp(&new_priority);
        self.entry.priority = new_priority;

        order
    }

    const fn ref_entry(&self) -> &Entry<P, I> {
        &self.entry
    }

    fn into_entry(self) -> Entry<P, I> {
        self.entry
    }

    const fn replace_parent(&mut self, tag: Tag) -> Option<Tag> {
        self.parent.replace(tag)
    }

    const fn take_parent(&mut self) -> Option<Tag> {
        self.parent.take()
    }

    const fn replace_child(&mut self, tag: Tag) -> Option<Tag> {
        self.parent.replace(tag)
    }

    const fn take_child(&mut self) -> Option<Tag> {
        self.child.take()
    }

    fn has_child(&self, tag: Tag) -> bool {
        self.child.is_some_and(|child_tag| child_tag == tag)
    }

    const fn replace_sibling(&mut self, tag: Tag) -> Option<Tag> {
        self.sibling.replace(tag)
    }

    const fn take_sibling(&mut self) -> Option<Tag> {
        self.sibling.take()
    }

    fn has_sibling(&self, tag: Tag) -> bool {
        self.sibling.is_some_and(|sibling_tag| sibling_tag == tag)
    }
}
