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

    pub fn remove(&mut self, tuple_template: &Tuple) {
        self.space.remove(tuple_template)
    }

    fn find(&self, tuple_template: &Tuple) -> Option<Tuple> {
        self.space.find(tuple_template)
    }

    fn withdraw(&mut self, tuple_template: &Tuple) -> Option<Tuple> {
        self.space.withdraw(tuple_template)
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
    fn add(&mut self, tuple: Tuple) {
        match &self.root {
            Some(root) => {
                let found = Self::add_internal(Some(root.clone()), root.clone(), tuple);
                println!("Was this tuple already in space: {}", !found);
                if found {
                    self.size += 1;
                }
            }
            None => {
                self.root = Some(Rc::new(RefCell::new(TupleTrieNode::new(tuple))));
                self.size += 1
            }
        }
    }

    /// Removes a tuple matching a template from
    /// the tuple space.
    ///
    /// The trie is searched depth-first, so
    /// the occurance with most matching bytes
    /// in binary form is removed.
    fn remove(&self, tuple_template: &Tuple) {
        todo!("implement remove()")
    }

    fn find(&self, tuple_template: &Tuple) -> Option<Tuple> {
        todo!("implement find()")
    }

    fn withdraw(&mut self, tuple_template: &Tuple) -> Option<Tuple> {
        todo!("implement withdraw()")
    }
}

enum TreeNode {
    Left,
    Right,
}

impl TupleTrie {
    fn add_internal(
        node: Option<TupleTrieNodeRef>,
        parent: TupleTrieNodeRef,
        tuple: Tuple,
    ) -> bool {
        // println!("Node: {node:?}");
        // println!("Parent: {parent:?}");
        let mut parent = parent;
        let mut current_node = node;
        let mut tree_node = TreeNode::Left;

        while let Some(node) = current_node.clone() {
            match node.borrow().value.cmp_binary(&tuple) {
                Ordering::Greater => {
                    // println!("Going right");
                    current_node = node.borrow().clone().right;
                    tree_node = TreeNode::Right;
                }
                Ordering::Less => {
                    // println!("Going left");
                    current_node = node.borrow().clone().left;
                    tree_node = TreeNode::Left;
                }
                Ordering::Equal => return false,
            };
            parent = node.clone();
        }

        // println!("Inserting");
        match tree_node {
            TreeNode::Left => {
                parent.borrow_mut().left =
                    Some(Rc::new(RefCell::new(TupleTrieNode::new(tuple.clone()))))
            }
            TreeNode::Right => {
                parent.borrow_mut().right =
                    Some(Rc::new(RefCell::new(TupleTrieNode::new(tuple.clone()))))
            }
        }

        true
    }
}

impl ToString for TupleTrie {
    fn to_string(&self) -> String {
        todo!()
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
        ts.add(Tuple::new("t1"));
        ts.add(Tuple::new("t3"));
        ts.add(Tuple::new("a1"));
        ts.add(Tuple::new("t2"));
        ts.add(Tuple::new("t2"));

        println!("Tuple space: {ts:#?}");
    }
}
