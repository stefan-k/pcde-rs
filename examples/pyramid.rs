// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate pcde;
use pcde::Pyramid;

fn main() {
    let lim_x = (-8.0, 8.0);
    let lim_y = (-8.0, 8.0);
    let n_bins = (8, 8);
    let pyr = Pyramid::new(lim_x, lim_y, n_bins);
    println!("{:#?}", pyr);
}
