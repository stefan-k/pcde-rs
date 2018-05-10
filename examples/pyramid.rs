// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate pcde;
extern crate rand;
use pcde::Pyramid;
use rand::Rng;

fn main() {
    let lim_x = (-8.0, 8.0);
    let lim_y = (-8.0, 8.0);
    // let n_bins = (8, 8);
    let n_bins = (4, 4);
    println!("Building pyramid...");
    let mut pyr = Pyramid::new(lim_x, lim_y, n_bins);
    println!("Adding values...");

    for _ in 0..10000 {
        let x: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
        let y: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
        pyr.add_val(vec![x, y]);
    }
    //
    // pyr.add_val(vec![0.0, 0.0]);
    // pyr.add_val(vec![1.0, 0.5]);
    // println!("{:#?}", pyr);
    println!("{:#?}", pyr.get_layer(0));
    println!("{:#?}", pyr.get_layer(1));
    println!("{:#?}", pyr.get_layer(2));
    // println!("{:#?}", pyr.get_layer(3));
    // println!("{:#?}", pyr.get_layer(4));
}
