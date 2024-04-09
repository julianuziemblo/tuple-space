use crate::tuple::tuple::Tuple;

#[derive(Debug)]
pub struct TupleSpace {
    space: TupleTrie,
}

impl TupleSpace {
    pub fn new() -> Self {
        Self {
            space: TupleTrie::new(),
        }
    }

    pub fn add(&mut self, tuple: Tuple) {
        self.space.add(tuple)
    }
}

#[derive(Debug)]
struct TupleTrie {
    root: Option<*mut TupleTrieNode>,
}

impl TupleTrie {
    fn new() -> Self {
        Self { root: None }
    }

    fn add(&mut self, tuple: Tuple) {
        match self.root {
            Some(root) => {
                todo!();
            }
            None => self.root = Some(&mut TupleTrieNode::new(tuple)),
        }
    }
}

#[derive(Debug, Default)]
struct TupleTrieNode {
    left: Option<*mut TupleTrieNode>,
    right: Option<*mut TupleTrieNode>,
    value: Tuple,
}

impl TupleTrieNode {
    fn new(value: Tuple) -> Self {
        Self {
            left: None,
            right: None,
            value,
        }
    }
}
