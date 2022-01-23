/********************************************************************
* Copyright (c) 2021-2022, Eric Mackay
* All rights reserved.
* 
* This source code is licensed under the BSD-style license found in the
* LICENSE file in the root directory of this source tree.
********************************************************************/

use std::fmt;
use std::fs;
use crate::engine;
//use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};


/// Represents an individual entity in the game
/// including their shape
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprite {
    pixels: engine::Bitmap,
    pos: engine::Position,
}

impl Sprite {
    pub fn new(cols: usize, rows: usize, fg: u8, bg: u8, pos: Option<engine::Position>) -> Self {
        Self {
            pixels: engine::Bitmap::new(cols, rows, fg, bg),
            pos: match pos {
                Some(p) => p,
                None => engine::Position{ x: 0, y: 0 },
            },
        }
    }

    pub fn build_from_str(data: &str) -> Self {
        let sp: Sprite = serde_json::from_str(&data).expect("Could not deserialize Sprite");
        sp
    }

    pub fn build_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Could not read Sprite file");
        Sprite::build_from_str(&data)
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pixels.fmt(f)
    }
}



/// Represents the game board, all the sprites, and the actions that
/// can be taken by each of the sprites
#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    sprites: Vec<Sprite>,
    screen: engine::Bitmap,
}

impl Board {
    pub fn new(cols: usize, rows: usize, fg: u8, bg: u8) -> Self {
        Self {
            sprites: Vec::<Sprite>::new(),
            screen: engine::Bitmap::new(cols, rows, fg, bg),
        }
    }

    // TODO Write tests for these, which also helps document the expected JSON format
    pub fn build_from_str(data: &str) -> Self {
        let b: Board = serde_json::from_str(&data).expect("Could not deserialize Board");
        b
    }

    pub fn build_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Could not read Board file");
        Board::build_from_str(&data)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.screen.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /* Sprite tests */

    #[test]
    fn test_sprite_new() {
        let cols = 6;
        let rows = 3;
        let bg = 0;
        let fg = 1;
        let pos = Some(engine::Position{ x: 2, y: 3 });
        let s: Sprite = Sprite::new(cols, rows, fg, bg, pos);
        assert_eq!(s.pos.get_x(), 2);
        assert_eq!(s.pos.get_y(), 3);
    }

    #[test]
    fn test_sprite_new_default_pos() {
        let cols = 6;
        let rows = 3;
        let bg = 0;
        let fg = 1;
        let pos = None;
        let s: Sprite = Sprite::new(cols, rows, fg, bg, pos);
        assert_eq!(s.pos.get_x(), 0);
        assert_eq!(s.pos.get_y(), 0);
    }

    #[test]
    fn test_sprite_build_str() {
    	let data = r#"
        {
            "pixels": {
                "foreground": 4,
                "background": 1,
                "data": {
                    "rows": 2,
                    "cols": 3,
                    "elements": [
                        4, 1, 4,
                        1, 1, 4
                    ]
                }
            },
            "pos": {
            	"x": 2,
            	"y": 5
            }
        }
        "#;
        let sp = Sprite::build_from_str(data);
        assert_eq!(sp.pixels.get_fg(), 4);
        assert_eq!(sp.pixels.get_bg(), 1);
        assert_eq!(sp.pixels.get_data().get_cols(), 3);
        assert_eq!(sp.pixels.get_data().get_rows(), 2);
        assert_eq!(sp.pixels.get_data()[engine::Position{x: 0, y: 0}], 4);
        assert_eq!(sp.pixels.get_data()[engine::Position{x: 1, y: 0}], 1);
        assert_eq!(sp.pixels.get_data()[engine::Position{x: 2, y: 0}], 4);
        assert_eq!(sp.pixels.get_data()[engine::Position{x: 0, y: 1}], 1);
        assert_eq!(sp.pixels.get_data()[engine::Position{x: 1, y: 1}], 1);
        assert_eq!(sp.pixels.get_data()[engine::Position{x: 2, y: 1}], 4);
        assert_eq!(sp.pos.get_x(), 2);
        assert_eq!(sp.pos.get_y(), 5);
    }

    /* Board tests */

    #[test]
    fn test_board_new() {
        let cols = 6;
        let rows = 3;
        let bg = 0;
        let fg = 1;
        let _b: Board = Board::new(cols, rows, bg, fg);
    }
}
