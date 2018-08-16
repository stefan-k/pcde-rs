// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Node

use std::f64;
use std::sync::Arc;
use std::sync::RwLock;
use Extent;
use NodeRef;

#[derive(Debug)]
pub struct Node {
    id: u64,
    pub pos: Vec<f64>,
    val: f64,
    pub children: Vec<NodeRef>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
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

    pub fn id(&self) -> u64 {
        self.id
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
        Arc::new(RwLock::new(self))
    }

    pub fn inside(&self, pos: Vec<f64>, extent: Extent) -> bool {
        pos.iter()
            .zip(self.pos.iter())
            .zip(extent.iter())
            .map(|((&xn, xk), &l)| (xn >= xk - l) && (xn <= xk + l))
            .filter(|x| !x)
            .count()
            == 0
    }

    pub fn add(&mut self, pos: Vec<f64>, ext: Extent) -> &mut Self {
        // println!("{:?} x {:?} x {:?}", self.pos, pos, ext);
        self.val += self
            .pos
            .iter()
            .zip(pos.iter())
            .map(|(a, b)| (a - b).abs())
            .zip(ext.iter())
            .map(|(d, e)| 1.0 - d / e)
            .fold(f64::INFINITY, |a, b| a.min(b));
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.val = 0.0;
        self
    }
}
