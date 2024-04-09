use std::{cell::RefCell, rc::Rc};

use crate::tuple::tuple::Tuple;

type TupleTrieNodeRef = Rc<RefCell<TupleTrieNode>>;

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

    pub fn get_root_val(&self) -> Option<Tuple> {
        match &self.space.root {
            Some(root) => todo!(),
            None => todo!(),
        }
    }
}

#[derive(Debug)]
struct TupleTrie {
    root: Option<TupleTrieNodeRef>,
}

impl TupleTrie {
    fn new() -> Self {
        Self { root: None }
    }

    fn add(&mut self, tuple: Tuple) {
        match &self.root {
            Some(root) => {
                todo!();
            }
            None => self.root = Some(Rc::new(RefCell::new(TupleTrieNode::new(tuple)))),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct TupleTrieNode {
    left: Option<TupleTrieNodeRef>,
    right: Option<TupleTrieNodeRef>,
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

#[cfg(test)]
mod test {
    use crate::tuple::tuple::Tuple;

    use super::TupleSpace;

    #[test]
    fn test1() {
        let mut ts = TupleSpace::new();
        ts.add(Tuple::default());
        println!("Tuple space: {ts:?}");
    }
}
