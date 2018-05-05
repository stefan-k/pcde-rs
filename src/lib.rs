// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Point Cloud Density Estimation

mod pyramid;
// mod tree;

pub use pyramid::Pyramid;
// use tree::Node;

// pub fn build_tree(
//     lim_x: (f64, f64),
//     lim_y: (f64, f64),
//     n_bins: (usize, usize),
//     extent: (usize, usize),
// ) -> Node {
//     let bin_pos = bin_positions(lim_x, lim_y, n_bins, extent);
//     println!("{:?}", bin_pos);
//     unimplemented!()
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
