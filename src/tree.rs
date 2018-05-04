// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Tree

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;
type Extent = (f64, f64);

pub struct RootData {
    children: Vec<NodeRef>,
}

pub struct ChildData {
    children: Vec<NodeRef>,
    extent: Vec<Extent>,
}

pub struct BinData {
    pos: Vec<f64>,
    extent: Vec<Extent>,
    val: f64,
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
        }
    }

    pub fn push(&mut self, node: &NodeRef) -> &mut Self {
        match *node.borrow() {
            Node::Bin(_) | Node::Child(_) => {
                let node = node.clone();
                self.extent = node.borrow()
                    .extent()
                    .iter()
                    .zip(self.extent.iter())
                    .map(|(&(a, b), &(c, d))| {
                        (if a < c { a } else { c }, if b > d { b } else { d })
                    })
                    .collect();
                self.children.push(node);
            }
            _ => panic!("Can only push Node::Bin or Node::Child to Node::Child"),
        };
        self
    }

    pub fn extent(&self) -> Vec<Extent> {
        self.extent.clone()
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
        }
    }

    pub fn add(&mut self, val: f64) -> &mut Self {
        self.val += val;
        self
    }

    pub fn extent(&self) -> Vec<Extent> {
        self.extent.clone()
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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
