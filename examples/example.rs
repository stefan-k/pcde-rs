// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate pcde;
use pcde::build_tree;

fn main() {
    let lim_x = (-5.0, 5.0);
    let lim_y = (-5.0, 5.0);
    let n_bins = (10, 10);
    let extent = (1, 1);
    let tree = build_tree(lim_x, lim_y, n_bins, extent);
}
