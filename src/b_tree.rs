use std::cmp::Ordering;
use std::usize;

struct Entry<T: Clone, S: Eq + Ord> {
    value: T,
    key: S,
    next: Node<T, S>,
}

struct Node<T: Clone, S: Eq + Ord> {
    children: Vec<Entry<T, S>>,
    size: usize,
}

impl<T: Clone, S: Eq + Ord> Node<T, S> {
    pub fn new(size: usize) -> Node<T, S> {
        Node {
            children: Vec::with_capacity(size),
            size,
        }
    }
}

pub struct BTree<T: Clone, S: Eq + Ord> {
    root: Node<T, S>,
    degree: i16,
    height: i16,
    size: i16,
}

impl<T: Clone, S: Eq + Ord> BTree<T, S> {
    pub fn new(degree: i16) -> BTree<T, S> {
        BTree {
            root: Node::new(0),
            degree,
            height: 0,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn search(&self, x: &Node<T, S>, key: S, h: i16) -> Option<T> {
        if self.height == 0 {
            let children = &x.children;
            for child in children {
                if child.key == key {
                    return Option::Some(child.value.clone());
                }
            }
        } else {
            for (i, child) in x.children.iter().enumerate() {
                if i + 1 == x.size || x.children[i + 1].key.cmp(&key) == Ordering::Less {
                    return self.search(&child.next, key, h - 1);
                }
            }
        }
        None
    }

    pub fn get(&self, key: S) -> Option<T> {
        self.search(&self.root, key, self.height)
    }

    pub fn put(&mut self, key: S, value: T) {}
}
