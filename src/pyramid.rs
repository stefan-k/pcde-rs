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
type Extent = Vec<f64>;

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

    pub fn inside(&self, pos: Vec<f64>, extent: Extent) -> bool {
        pos.iter()
            .zip(self.pos.iter())
            .zip(extent.iter())
            .map(|((&xn, xk), &l)| (xn > xk - l) && (xn < xk + l))
            .filter(|x| !x)
            .count() == 0
    }
}

fn bin_positions(lim_x: (f64, f64), lim_y: (f64, f64), n_bins: (usize, usize)) -> Vec<(f64, f64)> {
    let step_x = (lim_x.0 + lim_x.1) / (n_bins.0 as f64);
    let step_y = (lim_y.0 + lim_y.1) / (n_bins.1 as f64);
    let mut out = Vec::with_capacity(n_bins.0 * n_bins.1);
    for xi in 0..n_bins.0 {
        for yi in 0..n_bins.1 {
            out.push((
                step_x / 2.0 + (xi as f64) * step_x,
                step_y / 2.0 + (yi as f64) * step_y,
            ));
        }
    }
    out
}

pub struct Pyramid {
    root: NodeRef,
    extents: Vec<Extent>,
}

impl Pyramid {
    pub fn new(
        (min_x, max_x): (f64, f64),
        (min_y, max_y): (f64, f64),
        (n_bins_x, n_bins_y): (u64, u64),
    ) -> Self {
        // first, create individual layers with their corresponding bin positions and extents
        // second, connect the layers properly
        assert!(n_bins_x.is_power_of_two());
        assert!(n_bins_y.is_power_of_two());
        // for now, assure that the number of bins are the same in all directions
        assert!(n_bins_x == n_bins_y);
        // let num_layers = [(n_bins_x as f64).log2(), (n_bins_y as f64).log2()].max();
        let num_layers = (n_bins_x as f64).log2() as u64;

        // create root node
        let (root_pos_x, root_pos_y) = ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
        let root = Node::new(vec![root_pos_x, root_pos_y]).as_ref();
        let root_ext = vec![(max_x - root_pos_x) / 2.0, (max_y - root_pos_y) / 2.0];
        let mut extents = vec![];
        extents.push(root_ext);
        let mut pyr = Pyramid { root, extents };

        // push second node
        let bin_pos = bin_positions((min_x, max_x), (min_y, max_y), (2, 2));

        for b in bin_pos.iter() {
            let bin = Node::new(vec![b.0, b.1]);
            pyr.push_node(&bin.as_ref(), 2);
        }

        pyr
    }

    pub fn push_node(&mut self, node: &NodeRef, layer: u64) -> &mut Self {
        let mut curr_nodes = vec![self.root.clone()];
        let mut next_nodes = vec![];
        for lay in 0..layer {
            for cn in curr_nodes.iter() {
                cn.borrow()
                    .children
                    .iter()
                    .filter(|c| {
                        c.borrow().inside(
                            node.borrow().pos.clone(),
                            vec![lay as f64 + 1.0, lay as f64 + 1.0],
                        )
                    })
                    .map(|c| next_nodes.push(c.clone()))
                    .count();
            }
        }
        self
    }
}
