/********************************************************************
* Copyright (c) 2021-2022, Eric Mackay
* All rights reserved.
*
* This source code is licensed under the BSD-style license found in the
* LICENSE file in the root directory of this source tree.
********************************************************************/

use crate::engine;
use std::fmt;
use std::fs;
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
                None => engine::Position { x: 0, y: 0 },
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

    pub fn get_pixels(&self) -> &engine::Bitmap {
        &self.pixels 
    }

    pub fn get_pos(&self) -> &engine::Position {
        &self.pos
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.pixels.fmt(f)
    }
}

#[derive(Debug)]
pub enum BoardError {
    PixelOccupied,
    PosOccupied,
    OutOfRange,
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

    pub fn build_from_str(data: &str) -> Self {
        let b: Board = serde_json::from_str(&data).expect("Could not deserialize Board");
        b
    }

    pub fn build_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Could not read Board file");
        Board::build_from_str(&data)
    }

    // Brute-force rescan of entire board (or maybe rescan just Sprites)
    // Pixels occupied by a Sprite are colored fg, and rest bg
    //
    // TODO Likely will be a major performance bottleneck in future
    // TODO Better idea in future is to only look at positions that
    //  a Sprite previously occupied
    pub fn update(&mut self) {
        self.screen.reset();
        
        for sprite in &self.sprites {
            for y in 0..sprite.pixels.get_data().get_rows() {
                for x in 0..sprite.pixels.get_data().get_cols() {
                    println!("y = {}, x = {}", y, x);
                    if self.screen.get_data()[engine::Position { x: (x+sprite.get_pos().get_x()), y: (y+sprite.get_pos().get_y()) }] != self.screen.get_bg() {
                        panic!("Failed to update board")
                    }
                    self.screen.get_data_mut()[engine::Position { x: (x+sprite.get_pos().get_x()), y: (y+sprite.get_pos().get_y()) }] = sprite.pixels.get_data()[engine::Position { x: x, y: y }]
                }

            }
        }

        // TODO update stuff
    }



    // TODO Detect sprite position conflicts?
    // Make a new bitmap for each, starting at the same upperleft and going to same bottom right
    // Sprite FG positions copied in?
    // Then iterate 1 bitmap and check other bitmap?
    // Or just convert using math. Probably way faster but easier to get wrong.
    pub fn add_sprite(&mut self, newsprite: Sprite) {
        self.sprites.push(newsprite);
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
        let pos = Some(engine::Position { x: 2, y: 3 });
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
        assert_eq!(sp.pixels.get_data()[engine::Position { x: 0, y: 0 }], 4);
        assert_eq!(sp.pixels.get_data()[engine::Position { x: 1, y: 0 }], 1);
        assert_eq!(sp.pixels.get_data()[engine::Position { x: 2, y: 0 }], 4);
        assert_eq!(sp.pixels.get_data()[engine::Position { x: 0, y: 1 }], 1);
        assert_eq!(sp.pixels.get_data()[engine::Position { x: 1, y: 1 }], 1);
        assert_eq!(sp.pixels.get_data()[engine::Position { x: 2, y: 1 }], 4);
        assert_eq!(sp.pos.get_x(), 2);
        assert_eq!(sp.pos.get_y(), 5);
    }

    /* Board tests */

    #[test]
    fn test_board_new() {
        let cols = 6;
        let rows = 3;
        let bg = 4;
        let fg = 9;
        let b: Board = Board::new(cols, rows, fg, bg);
        assert_eq!(b.sprites.len(), 0);
        let _ = serde_json::to_string(&b).expect("Could not stringify");
        for y_in in 0..rows {
            for x_in in 0..cols {
                assert_eq!(b.screen.get_data()[engine::Position { x: x_in, y: y_in }], bg);
            }
        }
    }

    #[test]
    fn test_board_new_push_sprites() {
        let cols = 6;
        let rows = 3;
        let bg = 0;
        let fg = 1;
        let mut b: Board = Board::new(cols, rows, fg, bg);
        let _ = &b.sprites.push(Sprite::new(6, 3, 4, 5, None));
        let _ = &b.sprites.push(Sprite::new(4, 2, 8, 7, None));
        assert_eq!(b.sprites.len(), 2);
        let _ = serde_json::to_string(&b).expect("Could not stringify");
        b.update()
        // TODO More asserts
        // TODO Check new fg and bg values
    }

    #[test]
    fn test_board_build_str() {
        let data = r#"
        {
            "sprites": [
                {
                    "pixels": {
                        "foreground": 4,
                        "background": 1,
                        "data": {
                            "rows": 2,
                            "cols": 3,
                            "elements": [
                                1, 4, 1,
                                1, 4, 1
                            ]
                        }
                    },
                    "pos": {
                        "x": 0,
                        "y": 0
                    }
                },
                {
                    "pixels": {
                        "foreground": 6,
                        "background": 7,
                        "data": {
                            "rows": 2,
                            "cols": 3,
                            "elements": [
                                7, 7, 6,
                                6, 7, 6
                            ]
                        }
                    },
                    "pos": {
                        "x": 0,
                        "y": 2
                    }
                }
            ],
            "screen": {
                "foreground": 1,
                "background": 0,
                "data": {
                    "rows": 4,
                    "cols": 3,
                    "elements": [
                        0, 1, 0,
                        1, 1, 0,
                        1, 0, 0,
                        1, 0, 0
                    ]
                }
            }
        }
        "#;
        let _ = Board::build_from_str(data);
        // TODO More asserts
    }

    #[test]
    fn test_board_update() {
        // TODO More asserts
    }
}
