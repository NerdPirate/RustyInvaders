/********************************************************************
* Copyright (c) 2021-2022, Eric Mackay
* All rights reserved.
* 
* This source code is licensed under the BSD-style license found in the
* LICENSE file in the root directory of this source tree.
********************************************************************/

use crate::engine::Bitmap;

mod engine;


fn main() {
    let bm = Bitmap::build_from_file("./src/test1.json");
    println!("Bitmap:\n{}", bm);
}
