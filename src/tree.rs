// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Tree

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;
type Extent = (f64, f64);

pub struct RootData {
    children: Vec<NodeRef>,
}

pub struct ChildData {
    children: Vec<NodeRef>,
    extent: Vec<Extent>,
    parent: Option<WeakNodeRef>,
}

pub struct BinData {
    pos: Vec<f64>,
    extent: Vec<Extent>,
    val: f64,
    parent: Option<WeakNodeRef>,
}

pub enum Node {
    Root(RootData),
    Child(ChildData),
    Bin(BinData),
}

impl RootData {
    pub fn new() -> Self {
        RootData { children: vec![] }
    }

    pub fn push(&mut self, node: &NodeRef) -> &mut Self {
        match *node.borrow() {
            Node::Child(_) => self.children.push(node.clone()),
            _ => panic!("Can only push Node::Child to Node::Root."),
        };
        self
    }
}

impl ChildData {
    pub fn new() -> Self {
        ChildData {
            children: vec![],
            extent: vec![],
            parent: None,
        }
    }

    pub fn push(&mut self, node: &NodeRef) -> &mut Self {
        match *node.borrow() {
            Node::Bin(_) | Node::Child(_) => {
                let node = node.clone();
                if self.extent.len() == 0 {
                    self.extent = node.borrow().extent();
                } else {
                    self.extent = node.borrow()
                        .extent()
                        .iter()
                        .zip(self.extent.iter())
                        .map(|(&(a, b), &(c, d))| {
                            (if a < c { a } else { c }, if b > d { b } else { d })
                        })
                        .collect();
                }
                self.children.push(node);
            }
            _ => panic!("Can only push Node::Bin or Node::Child to Node::Child"),
        };
        self
    }

    pub fn extent(&self) -> Vec<Extent> {
        self.extent.clone()
    }

    pub fn set_parent(&mut self, parent: &NodeRef) -> &mut Self {
        self.parent = Some(Rc::downgrade(parent));
        self
    }
}

impl BinData {
    pub fn new(pos: Vec<f64>, extent: Vec<Extent>) -> Self {
        assert_eq!(pos.len(), extent.len());
        let extent = extent
            .iter()
            .zip(pos.iter())
            .map(|(&(a, b), c)| (a + c, b + c))
            .collect();
        BinData {
            pos,
            extent,
            val: 0.0,
            parent: None,
        }
    }

    pub fn add(&mut self, val: f64) -> &mut Self {
        self.val += val;
        self
    }

    pub fn extent(&self) -> Vec<Extent> {
        self.extent.clone()
    }

    pub fn pos(&self) -> Vec<f64> {
        self.pos.clone()
    }

    pub fn set_parent(&mut self, parent: &NodeRef) -> &mut Self {
        self.parent = Some(Rc::downgrade(parent));
        self
    }
}

impl Node {
    pub fn new_root() -> Node {
        Node::Root(RootData::new())
    }

    pub fn new_child() -> NodeRef {
        Rc::new(RefCell::new(Node::Child(ChildData::new())))
    }

    pub fn new_bin(pos: Vec<f64>, extent: Vec<Extent>) -> NodeRef {
        Rc::new(RefCell::new(Node::Bin(BinData::new(pos, extent))))
    }

    pub fn add_to_bin(&mut self, val: f64) -> &mut Self {
        match *self {
            Node::Bin(ref mut b) => b.add(val),
            _ => panic!("Can only add values to bin."),
        };
        self
    }

    pub fn push_node(&mut self, node: &NodeRef) -> &mut Self {
        match *self {
            Node::Root(ref mut x) => {
                x.push(node);
                ()
            }
            Node::Child(ref mut x) => {
                x.push(node);
                ()
            }
            _ => panic!("Cannot set parent of Node::Root."),
        };
        self
    }

    pub fn set_parent(&mut self, parent: &NodeRef) -> &mut Self {
        match *self {
            Node::Bin(ref mut x) => {
                x.set_parent(parent);
                ()
            }
            Node::Child(ref mut x) => {
                x.set_parent(parent);
                ()
            }
            _ => panic!("Cannot push to Node::Bin."),
        };
        self
    }

    pub fn extent(&self) -> Vec<Extent> {
        match *self {
            Node::Bin(ref x) => x.extent(),
            Node::Child(ref x) => x.extent(),
            _ => panic!("No extent for Node::Root"),
        }
    }

    pub fn as_ref(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::{BinData, Node};

    #[test]
    fn create_bin() {
        let bin = BinData::new(vec![1.0, 2.0], vec![(-1.0, 1.0), (3.0, 4.0)]);
        assert_eq!(bin.extent(), vec![(0.0, 2.0), (5.0, 6.0)]);
        assert_eq!(bin.pos(), vec![1.0, 2.0]);
    }

    #[test]
    fn create_child_push_bin() {
        let child = Node::new_child();
        let bin = Node::new_bin(vec![1.0, 2.0], vec![(-1.0, 1.0), (3.0, 4.0)]);
        (*child.borrow_mut()).push_node(&bin);
        assert_eq!(child.borrow().extent(), vec![(0.0, 2.0), (5.0, 6.0)]);
    }

    #[test]
    fn create_child_push_bin_and_bin() {
        let child = Node::new_child();
        let bin1 = Node::new_bin(vec![1.0, 2.0], vec![(-1.0, 1.0), (3.0, 4.0)]);
        let bin2 = Node::new_bin(vec![1.0, 2.0], vec![(-8.0, 0.5), (2.0, 7.0)]);
        (*child.borrow_mut()).push_node(&bin1);
        (*child.borrow_mut()).push_node(&bin2);
        assert_eq!(child.borrow().extent(), vec![(-7.0, 2.0), (5.0, 9.0)]);
    }

    #[test]
    fn create_child_push_child_with_bin() {
        let child1 = Node::new_child();
        let child2 = Node::new_child();
        let bin = Node::new_bin(vec![1.0, 2.0], vec![(-1.0, 1.0), (3.0, 4.0)]);
        (*child1.borrow_mut()).push_node(&bin);
        (*child2.borrow_mut()).push_node(&child1);
        assert_eq!(child2.borrow().extent(), vec![(0.0, 2.0), (5.0, 6.0)]);
        // assert_eq!(child.pos(), vec![1.0, 2.0]);
    }
}
