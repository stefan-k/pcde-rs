// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Pyramid

use layer::diff_layer;
use std::f64;
use std::mem;
use Extent;
use Layer;
use Node;
use NodeRef;

fn bin_positions(lim: Vec<(f64, f64)>, n_bins: Vec<usize>) -> (Vec<Vec<f64>>, Extent) {
    let dims = n_bins.len();
    assert!(lim.len() == dims);
    let steps: Vec<f64> = lim
        .iter()
        .zip(n_bins.iter())
        .map(|((min, max), n_bin)| (max - min) / ((n_bin + 1) as f64))
        .collect();
    let tot_bins = n_bins.iter().fold(1, |acc, x| acc * x);

    let mut out: Vec<Vec<f64>> = Vec::with_capacity(tot_bins);
    match dims {
        1 => {
            for a in 0..n_bins[0] {
                out.push(vec![lim[0].0 + steps[0] * (1.0 + a as f64)]);
            }
        }
        2 => {
            for a in 0..n_bins[0] {
                let a_tmp = lim[0].0 + steps[0] * (1.0 + a as f64);
                for b in 0..n_bins[1] {
                    out.push(vec![a_tmp, lim[1].0 + steps[1] * (1.0 + b as f64)]);
                }
            }
        }
        3 => {
            for a in 0..n_bins[0] {
                let a_tmp = lim[0].0 + steps[0] * (1.0 + a as f64);
                for b in 0..n_bins[1] {
                    let b_tmp = lim[1].0 + steps[1] * (1.0 + b as f64);
                    for c in 0..n_bins[2] {
                        out.push(vec![a_tmp, b_tmp, lim[2].0 + steps[2] * (1.0 + c as f64)]);
                    }
                }
            }
        }
        4 => {
            for a in 0..n_bins[0] {
                let a_tmp = lim[0].0 + steps[0] * (1.0 + a as f64);
                for b in 0..n_bins[1] {
                    let b_tmp = lim[1].0 + steps[1] * (1.0 + b as f64);
                    for c in 0..n_bins[2] {
                        let c_tmp = lim[2].0 + steps[2] * (1.0 + c as f64);
                        for d in 0..n_bins[3] {
                            out.push(vec![
                                a_tmp,
                                b_tmp,
                                c_tmp,
                                lim[3].0 + steps[3] * (1.0 + d as f64),
                            ]);
                        }
                    }
                }
            }
        }
        _ => unimplemented!(),
    }

    (out, steps)
}

#[derive(Debug, Clone)]
pub struct Pyramid {
    root: NodeRef,
    pub layers: Vec<Layer>,
    limits: Vec<(f64, f64)>,
    n_bins: Vec<usize>,
}

/// compute the norm of two `Pyramid`s
pub fn norm(p1: &Pyramid, p2: &Pyramid) -> f64 {
    assert!(p1 == p2);
    p1.layers
        .iter()
        .skip(1)
        .zip(p2.layers.iter().skip(1))
        .map(|(a, b)| diff_layer(a, b))
        .collect::<Vec<Vec<f64>>>()
        .iter()
        .flat_map(|arr| arr.iter())
        .map(|x| x.powi(2))
        .sum::<f64>()
        .sqrt()
}

impl PartialEq for Pyramid {
    /// Pyramids are equal if they have the same strcture. This may be stupid.
    fn eq(&self, other: &Pyramid) -> bool {
        self.limits == other.limits && self.n_bins == other.n_bins
    }
}

impl Pyramid {
    pub fn new(limits: &Vec<(f64, f64)>, n_bins: &Vec<usize>) -> Self {
        // Bins need to be a power of two
        n_bins.iter().for_each(|x| assert!(x.is_power_of_two()));

        let num_layers = n_bins
            .iter()
            .map(|b| (*b as f64).log2() as u32)
            .max()
            .unwrap();

        // create root node
        let root_pos: Vec<f64> = limits.iter().map(|(min, max)| (min + max) / 2.0).collect();
        let root = Node::new(root_pos.clone(), 0).as_ref();
        let root_ext: Vec<f64> = limits
            .iter()
            .zip(root_pos.iter())
            .map(|((_, max), pos)| 2.0 * (max - pos))
            .collect();

        let mut root_layer = Layer::new(0, root_ext);
        root_layer.push_node(&root);

        let mut pyr = Pyramid {
            root,
            layers: Vec::with_capacity(num_layers as usize),
            limits: limits.clone(),
            n_bins: n_bins.clone(),
        };
        pyr.push_layer(root_layer);

        for l in 1..(num_layers + 1) {
            let (bin_pos, ext) = bin_positions(
                limits.clone(),
                (0..(n_bins.len()))
                    .into_iter()
                    .map(|_| (l as f64).exp2() as usize)
                    .collect(),
            );
            // bin_positions(limits.clone(), vec![2_usize.pow(l), 2_usize.pow(l)]);
            // bin_positions(limits.clone(), (0..(bins.len())).into_iter().map(|_| 2_usize.pow(l)).collect());

            let mut layer = Layer::new(l as usize, ext);

            for (id, b) in bin_pos.into_iter().enumerate() {
                let bin = Node::new(b, id as u64).as_ref();
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

    pub fn layer(&self, layer: usize) -> Layer {
        self.layers[layer].clone()
    }

    pub fn num_layers(&self) -> usize {
        self.layers.len()
    }

    pub fn push_node(&mut self, node: &NodeRef, layer: u64) -> &mut Self {
        let mut curr_nodes = vec![self.root.clone()];
        let mut next_nodes: Vec<NodeRef> = Vec::with_capacity(200);
        let npos = node.read().unwrap().pos.clone();
        for lay in 0..layer {
            let ext = self.extent_of_layer((lay + 1) as usize);
            for cn in curr_nodes.iter() {
                cn.read()
                    .unwrap()
                    .children
                    .iter()
                    .filter(|c| c.read().unwrap().inside(npos.clone(), ext.clone()))
                    .map(|c| {
                        let c_id = c.read().unwrap().id();
                        if !next_nodes
                            .iter()
                            .map(|x| x.read().unwrap().id() == c_id)
                            .fold(false, |acc, x| acc | x)
                        {
                            next_nodes.push(c.clone())
                        }
                    }).count();
            }
            mem::swap(&mut curr_nodes, &mut next_nodes);
            next_nodes.clear();
        }
        for last_node in curr_nodes.iter() {
            last_node.write().unwrap().push_child(&node);
        }
        self
    }

    fn extent_of_layer(&mut self, layer: usize) -> Extent {
        let n_bins = 2_usize.pow(layer as u32);
        // let steps: Vec<f64> = lim
        //     .iter()
        //     .zip(n_bins.iter())
        //     .map(|((min, max), n_bin)| (max - min) / ((n_bin + 1) as f64))
        //     .collect();
        let step_x = (self.limits[0].1 - self.limits[0].0) / ((n_bins + 1) as f64);
        let step_y = (self.limits[1].1 - self.limits[1].0) / ((n_bins + 1) as f64);
        vec![step_x, step_y]
    }

    pub fn add_point(&mut self, pos: Vec<f64>) -> &mut Self {
        let mut curr_nodes = vec![self.root.clone()];
        let mut next_nodes: Vec<NodeRef> = vec![];
        for lay in 0..self.layers.len() {
            let ext = self.extent_of_layer((lay + 1) as usize);
            for cn in curr_nodes.iter() {
                cn.read()
                    .unwrap()
                    .children
                    .iter()
                    .filter(|c| c.read().unwrap().inside(pos.clone(), ext.clone()))
                    .map(|c| {
                        let c_id = c.read().unwrap().id();
                        if !next_nodes
                            .iter()
                            .map(|x| x.read().unwrap().id() == c_id)
                            .fold(false, |acc, x| acc | x)
                        {
                            next_nodes.push(c.clone())
                        }
                    }).count();
            }
            next_nodes
                .iter()
                .map(|x| {
                    let mut y = x.write().unwrap();
                    y.add(pos.clone(), ext.clone());
                }).count();
            mem::swap(&mut curr_nodes, &mut next_nodes);
            next_nodes.clear();
        }
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        for lay in self.layers.iter() {
            for y in lay.node.iter() {
                let mut a = y.write().unwrap();
                a.clear();
            }
        }
        self
    }
}
