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
    // let n_bins = vec![2, 2];
    // let n_bins = vec![4, 4];
    // let n_bins = vec![8, 8];
    // let n_bins = vec![16, 16];
    // let n_bins = vec![32, 32];
    // let n_bins = vec![64, 64];
    // let n_bins = vec![128, 128];
    let n_bins = vec![256, 256];
    println!("Building pyramid...");
    let mut pyr = Pyramid::new(vec![lim_x, lim_y], n_bins);
    println!("Adding values...");

    for _ in 0..(256 * 256) {
        let x: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
        let y: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
        pyr.add_point(vec![x, y]);
    }
    println!("{:#?}", pyr.layer(2).values());
    pyr.layer(7).write_map("bla.bmp");
    pyr.clear();
    println!("{:#?}", pyr.layer(2).values());
}
