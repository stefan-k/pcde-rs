// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Point Cloud Density Estimation

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;
type Extent = (f64, f64);

pub enum Node {
    Bin {
        pos: Vec<f64>,
        val: f64,
    },
    Child {
        bins: Vec<NodeRef>,
        extent: Vec<Extent>,
    },
}

impl Node {
    pub fn new_bin(pos: Vec<f64>) -> Self {
        Node::Bin { pos, val: 0.0 }
    }

    pub fn add_to_bin(&mut self, val: f64) -> &mut Self {
        match *self {
            Node::Bin {
                pos: _,
                val: ref mut v,
            } => *v += val,
            _ => panic!("Can only add values to bin."),
        };
        self
    }

    pub fn pos(&self) -> Vec<f64> {
        match *self {
            Node::Bin { pos: ref p, .. } => p.clone(),
            _ => panic!("only applicable to bins"),
        }
    }

    pub fn new_child() -> Self {
        Node::Child {
            bins: vec![],
            extent: vec![],
        }
    }

    pub fn push_node(&mut self, node: &NodeRef, extent: Vec<Extent>) -> &mut Self {
        match *node.borrow() {
            Node::Bin { .. } => {
                match *self {
                    Node::Child {
                        bins: ref mut b,
                        extent: ref mut ext,
                    } => {
                        let bin = node.clone();
                        assert_eq!(bin.borrow().len(), extent.len());
                        if b.len() == 0 {
                            *ext = extent
                                .iter()
                                .zip(bin.borrow().pos().iter())
                                .map(|(&(a, b), c)| (a + c, b + c))
                                .collect();
                        } else {
                            *ext = extent
                                .iter()
                                .zip(bin.borrow().pos().iter())
                                .map(|(&(a, b), c)| (a + c, b + c))
                                .zip(ext.iter())
                                .map(|((a1, b1), &(a2, b2))| {
                                    (
                                        if a1 < a2 { a1 } else { a2 },
                                        if b1 > b2 { b1 } else { b2 },
                                    )
                                })
                                .collect();
                        }
                        b.push(bin);
                    }
                    Node::Bin { .. } => panic!("Cannot push Bin into Bin."),
                };
            }
            Node::Child { .. } => match *self {
                Node::Child { .. } => unimplemented!(),
                Node::Bin { .. } => panic!("You are trying to push a Child into a Bin. Unfortunately, this is not allowed."),
            },
        }
        self
    }

    pub fn len(&self) -> usize {
        match *self {
            Node::Bin { pos: ref p, .. } => p.len(),
            Node::Child { bins: ref b, .. } => b.len(),
        }
    }

    pub fn as_ref(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }
}

// struct Node {

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
