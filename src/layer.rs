// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Layer

use image;
use Extent;
use NodeRef;

#[derive(Debug, Clone)]
pub struct Layer {
    pub node: Vec<NodeRef>,
    extent: Extent,
    bins: Vec<usize>,
}

/// calculate the difference between two layers
pub fn diff_layer(l1: &Layer, l2: &Layer) -> Vec<f64> {
    l1.node
        .iter()
        .zip(l2.node.iter())
        .map(|(a, b)| a.read().unwrap().val() - b.read().unwrap().val())
        .collect()
}

impl Layer {
    pub fn new(layer: usize, extent: Extent) -> Self {
        Layer {
            node: Vec::with_capacity(layer.pow(2) * layer.pow(2)),
            extent,
            bins: vec![2usize.pow(layer as u32), 2usize.pow(layer as u32)],
        }
    }

    pub fn push_node(&mut self, node: &NodeRef) -> &mut Self {
        self.node.push(node.clone());
        self
    }

    pub fn values(&self) -> Vec<f64> {
        self.node.iter().map(|x| x.read().unwrap().val()).collect()
    }

    pub fn write_map(&self, file: &str) {
        let img: Vec<f64> = self.node.iter().map(|x| x.read().unwrap().val()).collect();
        // let img_min = img.iter().cloned().fold(0. / 0., f64::min);
        // let img: Vec<f64> = img.iter().map(|x| x - img_min).collect();
        let img_max = img.iter().cloned().fold(0. / 0., f64::max);
        let img: Vec<u8> = img.iter().map(|x| (255.0 * x / img_max) as u8).collect();
        // println!("{:?}", img.len());
        // println!("{:?}", self.bins);
        // println!("{:?}", img);
        image::save_buffer(
            file,
            &img,
            self.bins[0] as u32,
            self.bins[1] as u32,
            image::Gray(8),
        ).unwrap();
    }
}
