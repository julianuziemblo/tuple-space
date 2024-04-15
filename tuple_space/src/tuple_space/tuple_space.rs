use std::{cell::RefCell, cmp::Ordering, rc::Rc};

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
        println!("Adding tuple {tuple:?}");
        self.space.add(tuple)
    }

    pub fn get_root_val(&self) -> Option<Tuple> {
        self.space
            .root
            .as_ref()
            .map(|root| root.borrow().clone().value)
    }

    pub fn size(&self) -> usize {
        self.space.size
    }
}

impl Default for TupleSpace {
    fn default() -> Self {
        Self {
            space: TupleTrie::new(),
        }
    }
}

#[derive(Debug)]
struct TupleTrie {
    root: Option<TupleTrieNodeRef>,
    size: usize,
}

type TupleTrieNodeRef = Rc<RefCell<TupleTrieNode>>;

impl TupleTrie {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// plan działania:
    /// przejście po drzewie:
    ///   - jeśli tuple jest "większe" niż posiadane to idzie na lewo
    ///   - jak mniejsze - na prawo.
    /// przejście: patrzymy, czy node istnieje:
    ///   - jeśli tak, to patrzymy, czy jest większy czy mniejszy, i tam przechodzimy
    ///   - jeśli nie, to dodajemy tam wartość
    /// ZASTANOWIĆ SIĘ: funkcja zwraca głębokość, na którą weszła
    pub fn add(&mut self, tuple: Tuple) {
        match &self.root {
            Some(root) => {
                let found = Self::add_internal(Some(root.clone()), root.clone(), tuple);
                println!("Was this tuple already in space: {}", !found);
                if !found {
                    self.size += 1;
                }
            }
            None => self.root = Some(Rc::new(RefCell::new(TupleTrieNode::new(tuple)))),
        }
    }

    pub fn remove(&self, tuple_template: &Tuple) {
        todo!()
    }

    pub fn find(&self, tuple_template: &Tuple) {
        todo!()
    }
}

impl TupleTrie {
    fn add_internal(
        node: Option<TupleTrieNodeRef>,
        parent: TupleTrieNodeRef,
        tuple: Tuple,
    ) -> bool {
        println!("Node: {node:?}");
        println!("Parent: {parent:?}");
        let mut parent = parent;
        let mut current_node = node;
        while let Some(node) = current_node.clone() {
            match node.borrow().value.cmp_binary(&tuple) {
                Ordering::Greater => {
                    println!("Going right");
                    current_node = node.borrow().clone().right;
                }
                Ordering::Less => {
                    println!("Going left");
                    current_node = node.borrow().clone().left;
                }
                Ordering::Equal => return false,
            };
            parent = node.clone();
        }

        println!("Inserting");
        parent.borrow_mut().left = Some(Rc::new(RefCell::new(TupleTrieNode::new(tuple.clone()))));
        true
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
    use crate::{
        tuple::tuple::Tuple,
        util::{Serializable, SliceU8},
    };

    use super::TupleSpace;

    #[test]
    fn mem_layout_test() {
        println!(
            "tuple default serialized: {:b}",
            SliceU8(&Tuple::default().serialize())
        );
        println!(
            "tuple t1 serialized: {:b}",
            SliceU8(&Tuple::new("t1").serialize())
        );
    }

    #[test]
    fn test1() {
        let mut ts = TupleSpace::new();
        ts.add(Tuple::default());
        ts.add(Tuple::new("t1"));
        ts.add(Tuple::new("t3"));
        ts.add(Tuple::new("a1"));
        ts.add(Tuple::new("t2"));
        ts.add(Tuple::new("t2"));

        println!("Tuple space: {ts:?}");
    }
}
