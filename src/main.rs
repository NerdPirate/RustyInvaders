/********************************************************************
* Copyright (c) 2021-2022, Eric Mackay
* All rights reserved.
* 
* This source code is licensed under the BSD-style license found in the
* LICENSE file in the root directory of this source tree.
********************************************************************/

use std::fs;
use crate::engine::Bitmap;

mod engine;
mod game;


fn main() {
    let data = fs::read_to_string("./src/bitmap.json").expect("Could not read file");
    let bd: Bitmap = serde_json::from_str(&data).expect("Could not parse json using derived code");
    println!("Derived Bitmap:\n{}", bd);
}
