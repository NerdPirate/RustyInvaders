/********************************************************************
* Copyright (c) 2021-2022, Eric Mackay
* All rights reserved.
*
* This source code is licensed under the BSD-style license found in the
* LICENSE file in the root directory of this source tree.
********************************************************************/

use crate::engine;
use std::cmp;
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
    bounds: engine::Rectangle,
}

impl Sprite {
    pub fn new(cols: usize, rows: usize, fg: u8, bg: u8, pos: Option<engine::Position>) -> Self {
        let new_pos = match pos {
                Some(p) => p,
                None => engine::Position { x: 0, y: 0 },
        };

        Self {
            pixels: engine::Bitmap::new(cols, rows, fg, bg),
            pos: new_pos,
            bounds: engine::Rectangle {
                top_left: engine::Position { x: new_pos.x, y: new_pos.y },
                // Bottom-right corner of 0-indexed rectangle, remember the -1
                bottom_right: engine::Position { x: new_pos.x + cols - 1, y: new_pos.y + rows - 1 },
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

    pub fn get_pixel_at(&self, point: &engine::Position) -> Option<u8> {
        // Bounds check
        if point.get_x() < self.pos.get_x() || point.get_y() < self.pos.get_y() {
            None
        } else {
            // Normalize the point, so we can redefine the Sprite's position as 0,0
            let normal_point = engine::Position { x: self.pos.get_x() - point.get_x(),
                y: self.pos.get_y() - point.get_y()
            };
            match self.pixels.get_data().in_range(&normal_point) {
                true => Some(self.pixels.get_data()[normal_point]),
                false => None,
            }
        }
    }

    pub fn get_bounds(&self) -> &engine::Rectangle {
        &self.bounds
    }

    pub fn intersect(&self, other: &Sprite) -> bool {
        // An exercise in destructuring
        let Sprite {
            bounds: engine::Rectangle {
                top_left: engine::Position {
                    x: self_left_side,
                    y: self_top_side,
                },
                bottom_right: engine::Position {
                    x: self_right_side,
                    y: self_bottom_side,
                }
            }, .. } = self;
        let Sprite {
            bounds: engine::Rectangle {
                top_left: engine::Position {
                    x: other_left_side,
                    y: other_top_side,
                },
                bottom_right: engine::Position {
                    x: other_right_side,
                    y: other_bottom_side,
                }
            }, .. } = other;

        // Check ranges first. Non-overlapping range is a faster calculation.
        if  other_left_side > self_right_side ||
            other_top_side > self_bottom_side ||
            other_right_side < self_left_side ||
            other_bottom_side < self_top_side {
                println!("False");
                false
        } else {
            println!("True");
            // Find overlapping range
            let common_start_x = cmp::max(self_left_side, other_left_side);
            let common_start_y = cmp::max(self_top_side, other_top_side);
            let common_end_x = cmp::min(self_right_side, other_right_side);
            let common_end_y = cmp::min(self_bottom_side, other_bottom_side);

            for common_y in *common_start_y..=*common_end_y {
                for common_x in *common_start_x..=*common_end_x {
                    let self_pixel = self.get_pixel_at(&engine::Position{ x: common_x, y: common_y });
                    let other_pixel = other.get_pixel_at(&engine::Position{ x: common_x, y: common_y });
                    let both_fg = match (self_pixel, other_pixel) {
                        (Some(sp), Some(op)) => sp == self.pixels.get_fg() && op == other.pixels.get_fg(),
                        (_,_) => false,
                    };
                    if both_fg {
                        return true;
                    }
                }
            }
            false
        }
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
        if self.sprites.len() > 0 {
            for sprite in self.sprites.iter() {
                if sprite.intersect(&newsprite) {
                    println!("REJECTED!");
                    return
                }
            }
        }
        println!("Adding");
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
            },
            "bounds": {
                "top_left": {
                    "x": 2,
                    "y": 5
                },
                "bottom_right": {
                    "x": 4,
                    "y": 6
                }
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

    #[test]
    fn test_sprite_intersect_ranges() {
        let data1 = r#"
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
            },
            "bounds": {
                "top_left": {
                    "x": 2,
                    "y": 5
                },
                "bottom_right": {
                    "x": 4,
                    "y": 6
                }
            }
        }
        "#;
        let sp1 = Sprite::build_from_str(data1);
        let sp1_again = Sprite::build_from_str(data1);
        let data2 = r#"
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
                "x": 5,
                "y": 5
            },
            "bounds": {
                "top_left": {
                    "x": 5,
                    "y": 5
                },
                "bottom_right": {
                    "x": 7,
                    "y": 6
                }
            }
        }
        "#;
        let sp2 = Sprite::build_from_str(data2);
        assert_eq!(sp1.intersect(&sp1_again), true);
        assert_eq!(sp1.intersect(&sp2), false);
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
    #[should_panic]
    fn test_board_new_push_sprites_fail() {
        let cols = 6;
        let rows = 3;
        let bg = 0;
        let fg = 1;
        let mut b: Board = Board::new(cols, rows, fg, bg);
        let _ = &b.add_sprite(Sprite::new(6, 3, 4, 5, None));
        // These overlap
        let _ = &b.add_sprite(Sprite::new(4, 2, 8, 7, None));
        assert_eq!(b.sprites.len(), 2);
        let _ = serde_json::to_string(&b).expect("Could not stringify");
        b.update()
        // TODO More asserts
        // TODO Check new fg and bg values
    }


    #[test]
    fn test_board_new_push_sprites_succeed() {
        let cols = 6;
        let rows = 3;
        let bg = 0;
        let fg = 1;
        let mut b: Board = Board::new(cols, rows, fg, bg);
        let _ = &b.add_sprite(Sprite::new(2, 3, 4, 5, None));
        // These don't overlap
        let _ = &b.add_sprite(Sprite::new(2, 3, 8, 7, Some(engine::Position { x: 2, y: 0 })));
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
                    },
                    "bounds": {
                        "top_left": {
                            "x": 0,
                            "y": 0
                        },
                        "bottom_right": {
                            "x": 2,
                            "y": 1
                        }
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
                    },
                    "bounds": {
                        "top_left": {
                            "x": 0,
                            "y": 2
                        },
                        "bottom_right": {
                            "x": 2,
                            "y": 3
                        }
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
