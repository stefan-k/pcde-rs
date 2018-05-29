// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO

#![feature(test)]
#![feature(concat_idents)]

extern crate pcde;
extern crate rand;
extern crate test;

#[cfg(test)]
mod tests {
    use pcde::Pyramid;
    use rand;
    use rand::Rng;
    use test::{black_box, Bencher};

    // #[bench]
    // fn pyramid_8(b: &mut Bencher) {
    //     let lim_x = (-8.0, 8.0);
    //     let n_bins = 8;
    //     // println!("{} {}", vec![lim_x].len(), n_bins.len());
    //     b.iter(|| {
    //         black_box(Pyramid::new(vec![lim_x], vec![n_bins]));
    //     });
    // }

    #[bench]
    fn pyramid_8x8(b: &mut Bencher) {
        let lim_x = (-8.0, 8.0);
        let lim_y = (-8.0, 8.0);
        let n_bins = vec![8, 8];
        b.iter(|| {
            black_box(Pyramid::new(vec![lim_x, lim_y], n_bins.clone()));
        });
    }

    #[bench]
    fn pyramid_64x64(b: &mut Bencher) {
        let lim_x = (-8.0, 8.0);
        let lim_y = (-8.0, 8.0);
        let n_bins = vec![64, 64];
        b.iter(|| {
            black_box(Pyramid::new(vec![lim_x, lim_y], n_bins.clone()));
        });
    }

    // #[bench]
    // fn pyramid_256x256(b: &mut Bencher) {
    //     let lim_x = (-8.0, 8.0);
    //     let lim_y = (-8.0, 8.0);
    //     let n_bins = (256, 256);
    //     b.iter(|| {
    //         black_box(Pyramid::new(lim_x, lim_y, n_bins));
    //     });
    // }

    #[bench]
    fn pyramid_64x64_add_point(b: &mut Bencher) {
        let lim_x = (-8.0, 8.0);
        let lim_y = (-8.0, 8.0);
        let n_bins = vec![64, 64];
        let mut pyr = Pyramid::new(vec![lim_x, lim_y], n_bins);
        b.iter(|| {
            let x: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
            let y: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
            black_box(pyr.add_val(vec![x, y]));
        });
    }

    #[bench]
    fn pyramid_256x256_add_point(b: &mut Bencher) {
        let lim_x = (-8.0, 8.0);
        let lim_y = (-8.0, 8.0);
        let n_bins = vec![256, 256];
        let mut pyr = Pyramid::new(vec![lim_x, lim_y], n_bins);
        b.iter(|| {
            let x: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
            let y: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
            black_box(pyr.add_val(vec![x, y]));
        });
    }

    #[bench]
    fn pyramid_512x512_add_point(b: &mut Bencher) {
        let lim_x = (-8.0, 8.0);
        let lim_y = (-8.0, 8.0);
        let n_bins = vec![512, 512];
        let mut pyr = Pyramid::new(vec![lim_x, lim_y], n_bins);
        b.iter(|| {
            let x: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
            let y: f64 = rand::thread_rng().gen_range(-8.0, 8.0);
            black_box(pyr.add_val(vec![x, y]));
        });
    }

    // CLEAR
    #[bench]
    fn pyramid_64x64_clear(b: &mut Bencher) {
        let lim_x = (-8.0, 8.0);
        let lim_y = (-8.0, 8.0);
        let n_bins = vec![64, 64];
        let mut pyr = Pyramid::new(vec![lim_x, lim_y], n_bins);
        b.iter(|| {
            black_box(pyr.clear());
        });
    }

    #[bench]
    fn pyramid_256x256_clear(b: &mut Bencher) {
        let lim_x = (-8.0, 8.0);
        let lim_y = (-8.0, 8.0);
        let n_bins = vec![256, 256];
        let mut pyr = Pyramid::new(vec![lim_x, lim_y], n_bins);
        b.iter(|| {
            black_box(pyr.clear());
        });
    }

    #[bench]
    fn pyramid_512x512_clear(b: &mut Bencher) {
        let lim_x = (-8.0, 8.0);
        let lim_y = (-8.0, 8.0);
        let n_bins = vec![512, 512];
        let mut pyr = Pyramid::new(vec![lim_x, lim_y], n_bins);
        b.iter(|| {
            black_box(pyr.clear());
        });
    }
}
