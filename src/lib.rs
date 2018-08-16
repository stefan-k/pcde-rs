// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Point Cloud Density Estimation

extern crate image;

use std::sync::Arc;
use std::sync::RwLock;

mod layer;
mod node;
mod pyramid;

use layer::Layer;
use node::Node;
pub use pyramid::norm;
pub use pyramid::Pyramid;

type NodeRef = Arc<RwLock<Node>>;
type Extent = Vec<f64>;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
