// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Point Cloud Density Estimation

mod pyramid;
mod tree;

use tree::Node;

fn bin_positions(
    lim_x: (f64, f64),
    lim_y: (f64, f64),
    n_bins: (usize, usize),
    extent: (usize, usize),
) -> Vec<(f64, f64)> {
    let border_x = (
        lim_x.0 + (extent.0 as f64) / 2.0,
        lim_x.1 - (extent.0 as f64) / 2.0,
    );
    let border_y = (
        lim_y.0 + (extent.1 as f64) / 2.0,
        lim_y.1 - (extent.1 as f64) / 2.0,
    );
    let step_x = (border_x.1 - border_x.0) / ((n_bins.0 - 1) as f64);
    let step_y = (border_y.1 - border_y.0) / ((n_bins.1 - 1) as f64);
    let mut out = Vec::with_capacity(n_bins.0 * n_bins.1);
    for xi in 0..n_bins.0 {
        for yi in 0..n_bins.1 {
            out.push((
                border_x.0 + (xi as f64) * step_x,
                border_y.0 + (yi as f64) * step_y,
            ));
        }
    }
    out
}

pub fn build_tree(
    lim_x: (f64, f64),
    lim_y: (f64, f64),
    n_bins: (usize, usize),
    extent: (usize, usize),
) -> Node {
    let bin_pos = bin_positions(lim_x, lim_y, n_bins, extent);
    println!("{:?}", bin_pos);
    unimplemented!()
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
