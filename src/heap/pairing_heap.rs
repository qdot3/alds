use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    rc::{Rc, Weak},
};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct PairingHeap<I, P> {
    root: Option<Rc<NodeRef<I, P>>>,
    map: HashMap<I, Rc<NodeRef<I, P>>>,
}

impl<I: Hash + Eq + Clone, P: Ord> Default for PairingHeap<I, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<I: Hash + Eq + Clone, P: Ord> PairingHeap<I, P> {
    pub fn new() -> Self {
        Self {
            root: None,
            map: HashMap::default(),
        }
    }

    pub fn insert(&mut self, identifier: I, priority: P) -> bool {
        if self.map.contains_key(&identifier) {
            return false;
        }

        let node = Rc::new(RefCell::new(Node::new(identifier.clone(), priority)));

        self.map.insert(identifier, Rc::clone(&node));
        self.root = if let Some(root) = std::mem::take(&mut self.root) {
            Some(Node::meld(root, node))
        } else {
            Some(node)
        };

        true
    }

    pub fn remove(&mut self, identifier: I) -> Option<(I, P)> {
        if self
            .map
            .get(&identifier)
            .is_some_and(|node| node.borrow().is_root())
        {
            self.pop()
        } else if let Some(mut node) = self.map.remove(&identifier) {
            Node::detach(&mut node);

            if let Some(child) = Node::pair_and_detach_children(&mut node) {
                self.root = Some(Node::meld(std::mem::take(&mut self.root).unwrap(), child));
            }

            let node = Rc::into_inner(node).unwrap().into_inner();
            Some((node.identifier, node.priority))
        } else {
            None
        }
    }

    /// # Example
    ///
    /// ```
    /// use alds::heap::PairingHeap;
    ///
    /// let mut heap = PairingHeap::new();
    /// for i in 0..100 {
    ///     let id = i;
    ///     let priority = -i;
    ///     assert!(heap.insert(id, priority))
    /// }
    ///
    /// assert_eq!(
    ///     Vec::from_iter(std::iter::from_fn(|| heap.pop())),
    ///     Vec::from_iter((0..100).map(|i| (i, -i))),
    /// )
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(log *n*), amortized
    pub fn pop(&mut self) -> Option<(I, P)> {
        if let Some(mut root) = std::mem::take(&mut self.root) {
            assert!(root.borrow().is_root());
            assert!(self.map.remove(root.borrow().identifier()).is_some());

            self.root = Node::pair_and_detach_children(&mut root);
            assert!(self.root.as_ref().is_none_or(|c| c.borrow().is_root()));

            let root = Rc::into_inner(root).unwrap().into_inner();
            Some((root.identifier, root.priority))
        } else {
            None
        }
    }

    pub fn prioritise(&mut self, identifier: I, new_priority: P) -> bool {
        if self
            .map
            .get(&identifier)
            .is_some_and(|node| node.borrow().is_root())
        {
            let mut root = std::mem::take(&mut self.root).unwrap();

            if &new_priority >= root.borrow().priority() {
                root.borrow_mut().priority = new_priority;
            } else if let Some(child) = Node::pair_and_detach_children(&mut root) {
                root = Node::meld(root, child)
            } else {
                root.borrow_mut().priority = new_priority;
            }

            self.root = Some(root);

            true
        } else if let Some(mut node) = self.map.remove(&identifier) {
            Node::detach(&mut node);

            if node.borrow().priority() >= &new_priority {
                node.borrow_mut().priority = new_priority
            } else {
                todo!()
            }

            todo!()
        } else {
            false
        }
    }
}

#[test]
fn test_pop() {
    let mut heap = PairingHeap::new();

    assert!(heap.insert(0, -100));
    assert!(!heap.insert(0, 100));
    assert!(heap.insert(1, 1000));
    assert!(heap.insert(2, 2000));
    println!("{:?}\n", heap);

    assert_eq!(heap.pop(), Some((2, 2000)));
    assert_eq!(heap.pop(), Some((1, 1000)));
    assert_eq!(heap.pop(), Some((0, -100)));
    assert!(heap.pop().is_none());
}

type NodeRef<I, P> = RefCell<Node<I, P>>;

#[derive(Debug, Clone)]
struct Node<I, P> {
    identifier: I,
    priority: P,

    parent: Option<Weak<NodeRef<I, P>>>,
    child: Option<Rc<NodeRef<I, P>>>,
    sibling: Option<Rc<NodeRef<I, P>>>,
}

impl<I, P: Ord> Node<I, P> {
    fn new(identifier: I, priority: P) -> Self {
        Self {
            identifier,
            priority,
            parent: None,
            child: None,
            sibling: None,
        }
    }

    const fn identifier(&self) -> &I {
        &self.identifier
    }

    const fn priority(&self) -> &P {
        &self.priority
    }

    const fn is_root(&self) -> bool {
        self.parent.is_none() && self.sibling.is_none()
    }

    fn has_child(&self, node: &Rc<NodeRef<I, P>>) -> bool {
        self.child
            .as_ref()
            .is_some_and(|child| Rc::ptr_eq(child, node))
    }

    fn has_sibling(&self, node: &Rc<NodeRef<I, P>>) -> bool {
        self.sibling
            .as_ref()
            .is_some_and(|child| Rc::ptr_eq(child, node))
    }

    /// Detaches given node from the parent and siblings.
    fn detach(node: &mut Rc<NodeRef<I, P>>) {
        if let Some(parent) = std::mem::take(&mut node.borrow_mut().parent) {
            let parent = Weak::upgrade(&parent).unwrap();

            if parent.borrow().has_child(node) {
                parent.borrow_mut().child = std::mem::take(&mut node.borrow_mut().sibling);
            } else if parent.borrow().has_sibling(node) {
                parent.borrow_mut().sibling = std::mem::take(&mut node.borrow_mut().sibling);
            } else {
                unreachable!("given node should be a child or sibling of the parent")
            }
        }

        assert!(node.borrow().is_root())
    }

    fn pair_and_detach_children(node: &mut Rc<NodeRef<I, P>>) -> Option<Rc<NodeRef<I, P>>> {
        let mut child = std::mem::take(&mut node.borrow_mut().child);
        std::iter::from_fn(move || {
            std::mem::take(&mut child).inspect(|inner| {
                inner.borrow_mut().parent = None;
                let sibling = std::mem::take(&mut inner.borrow_mut().sibling);
                child = sibling;
            })
        })
        .chunks(2)
        .into_iter()
        .map(|pair| pair.reduce(|acc, node| Node::meld(acc, node)).unwrap())
        .reduce(|acc, node| Node::meld(acc, node))
    }

    fn meld(mut root_1: Rc<NodeRef<I, P>>, mut root_2: Rc<NodeRef<I, P>>) -> Rc<NodeRef<I, P>> {
        assert!(root_1.borrow().is_root());
        assert!(root_2.borrow().is_root());

        if root_1.borrow().priority() < root_2.borrow().priority() {
            std::mem::swap(&mut root_1, &mut root_2);
        }

        root_2.borrow_mut().sibling = std::mem::take(&mut root_1.borrow_mut().child);
        root_2.borrow_mut().parent = Some(Rc::downgrade(&root_1));
        root_1.borrow_mut().child = Some(root_2);

        root_1
    }
}
