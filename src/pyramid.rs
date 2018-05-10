// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Pyramid

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;
type Extent = Vec<f64>;

#[derive(PartialEq, Debug)]
pub struct Node {
    id: u64,
    pos: Vec<f64>,
    val: f64,
    children: Vec<NodeRef>,
}

impl Node {
    pub fn new(pos: Vec<f64>, id: u64) -> Node {
        Node {
            id,
            pos,
            val: 0.0,
            children: vec![],
        }
    }

    /// *giggles*
    pub fn push_child(&mut self, node: &NodeRef) -> &mut Self {
        self.children.push(node.clone());
        self
    }

    pub fn val(&self) -> f64 {
        self.val
    }

    pub fn as_ref(self) -> NodeRef {
        Rc::new(RefCell::new(self))
    }

    pub fn inside(&self, pos: Vec<f64>, extent: Extent) -> bool {
        pos.iter()
            .zip(self.pos.iter())
            .zip(extent.iter())
            .map(|((&xn, xk), &l)| (xn >= xk - l) && (xn <= xk + l))
            .filter(|x| !x)
            .count() == 0
    }

    pub fn add(&mut self, val: f64) -> &mut Self {
        self.val += val;
        self
    }
}

fn bin_positions(
    lim_x: (f64, f64),
    lim_y: (f64, f64),
    n_bins: (usize, usize),
) -> (Vec<(f64, f64)>, Extent) {
    let step_x = (lim_x.1 - lim_x.0) / ((n_bins.0 - 1) as f64);
    let step_y = (lim_y.1 - lim_y.0) / ((n_bins.1 - 1) as f64);
    let mut out = Vec::with_capacity(n_bins.0 * n_bins.1);
    for xi in 0..n_bins.0 {
        for yi in 0..n_bins.1 {
            out.push((
                lim_x.0 + step_x * (xi as f64),
                lim_y.0 + step_y * (yi as f64),
            ));
        }
    }
    (out, vec![step_x, step_y])
}

#[derive(Debug)]
pub struct Layer {
    node: Vec<NodeRef>,
    extent: Extent,
}

impl Layer {
    pub fn new(layer: usize, extent: Extent) -> Self {
        Layer {
            node: Vec::with_capacity(layer.pow(2) * layer.pow(2)),
            extent,
        }
    }

    pub fn push_node(&mut self, node: &NodeRef) -> &mut Self {
        self.node.push(node.clone());
        self
    }
}

#[derive(Debug)]
pub struct Pyramid {
    root: NodeRef,
    layers: Vec<Layer>,
    limits: Vec<(f64, f64)>,
}

impl Pyramid {
    pub fn new(
        (min_x, max_x): (f64, f64),
        (min_y, max_y): (f64, f64),
        (n_bins_x, n_bins_y): (u64, u64),
    ) -> Self {
        // first, create individual layers with their corresponding bin positions and extents
        // second, connect the layers properly

        // Bins need to be a power of two
        assert!(n_bins_x.is_power_of_two());
        assert!(n_bins_y.is_power_of_two());

        // for now, assure that the number of bins are the same in all directions
        assert!(n_bins_x == n_bins_y);

        // let num_layers = [(n_bins_x as f64).log2(), (n_bins_y as f64).log2()].max();
        let num_layers = (n_bins_x as f64).log2() as u32;

        // create root node
        let (root_pos_x, root_pos_y) = ((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
        let root = Node::new(vec![root_pos_x, root_pos_y], 0).as_ref();
        let root_ext = vec![2.0 * (max_x - root_pos_x), 2.0 * (max_y - root_pos_y)];
        let mut root_layer = Layer::new(0, root_ext);
        root_layer.push_node(&root);
        let limits = vec![(min_x, max_x), (min_y, max_y)];
        let mut pyr = Pyramid {
            root,
            layers: Vec::with_capacity(num_layers as usize),
            limits,
        };
        pyr.push_layer(root_layer);

        for l in 1..(num_layers + 1) {
            let (bin_pos, ext) = bin_positions(
                (min_x, max_x),
                (min_y, max_y),
                (2_usize.pow(l), 2_usize.pow(l)),
            );

            let mut layer = Layer::new(0, ext);

            for (id, b) in bin_pos.iter().enumerate() {
                let bin = Node::new(vec![b.0, b.1], id as u64).as_ref();
                layer.push_node(&bin);
                pyr.push_node(&bin, (l - 1).into());
            }
            pyr.push_layer(layer);
        }

        pyr
    }

    pub fn push_layer(&mut self, layer: Layer) -> &mut Self {
        self.layers.push(layer);
        self
    }

    pub fn get_layer(&self, layer: usize) -> Vec<f64> {
        self.layers[layer]
            .node
            .iter()
            .map(|x| x.borrow().val())
            .collect()
    }

    pub fn push_node(&mut self, node: &NodeRef, layer: u64) -> &mut Self {
        let mut curr_nodes = vec![self.root.clone()];
        let mut next_nodes: Vec<NodeRef> = vec![];
        for lay in 0..layer {
            for cn in curr_nodes.iter() {
                cn.borrow()
                    .children
                    .iter()
                    .filter(|c| {
                        c.borrow().inside(
                            node.borrow().pos.clone(),
                            self.extent_of_layer((lay + 1) as usize),
                        )
                    })
                    .map(|c| {
                        if !next_nodes.contains(c) {
                            next_nodes.push(c.clone())
                        }
                    })
                    .count();
            }
            mem::swap(&mut curr_nodes, &mut next_nodes);
            next_nodes.clear();
        }
        for last_node in curr_nodes.iter() {
            last_node.borrow_mut().push_child(&node);
        }
        self
    }

    fn extent_of_layer(&mut self, layer: usize) -> Extent {
        let n_bins = 2_usize.pow(layer as u32);
        let step_x = (self.limits[0].1 - self.limits[0].0) / ((n_bins - 1) as f64);
        let step_y = (self.limits[1].1 - self.limits[1].0) / ((n_bins - 1) as f64);
        vec![step_x, step_y]
    }

    pub fn add_val(&mut self, pos: Vec<f64>, val: f64) -> &mut Self {
        let mut curr_nodes = vec![self.root.clone()];
        let mut next_nodes: Vec<NodeRef> = vec![];
        for lay in 0..self.layers.len() {
            for cn in curr_nodes.iter() {
                cn.borrow()
                    .children
                    .iter()
                    .filter(|c| {
                        c.borrow()
                            .inside(pos.clone(), self.extent_of_layer((lay + 1) as usize))
                    })
                    .map(|c| {
                        if !next_nodes.contains(c) {
                            next_nodes.push(c.clone())
                        }
                    })
                    .count();
            }
            next_nodes
                .iter()
                .map(|x| {
                    let mut y = x.borrow_mut();
                    y.add(val);
                })
                .count();
            mem::swap(&mut curr_nodes, &mut next_nodes);
            next_nodes.clear();
        }
        self
    }
}
