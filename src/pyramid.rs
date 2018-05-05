// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Pyramid

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;
type Extent = Vec<(f64, f64)>;

pub struct Node {
    pos: Vec<f64>,
    val: f64,
    children: Vec<NodeRef>,
}

impl Node {
    pub fn new(pos: Vec<f64>) -> Node {
        Node {
            pos,
            val: 0.0,
            children: vec![],
        }
    }

    pub fn push_child(&mut self, node: &NodeRef) -> &mut Self {
        self.children.push(node.clone());
        self
    }

    pub fn as_ref(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }
}

pub struct Pyramid {
    root: Node,
    extents: Vec<Extent>,
}

impl Pyramid {
    pub fn new(
        (min_x, max_x): (f64, f64),
        (min_y, max_y): (f64, f64),
        (n_bins_x, n_bins_y): (u64, u64),
        (extent_x, extent_y): (u64, u64),
    ) -> Self {
        // todo: n_bins_x and n_bins_y must be a power of 2
        unimplemented!()
    }
}
