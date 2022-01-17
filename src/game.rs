use std::fmt;
use std::fs;
use crate::engine;
//use std::ops::{Index, IndexMut};
use serde_json::Value;


/// Represents an individual entity in the game
/// including their shape
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
        let mut local_sprite = Self {
            pixels: engine::Bitmap::build_from_str(data),
            pos: engine::Position{ x: 0, y: 0},
        };
        let v: Value = serde_json::from_str(&data).expect("Could not parse data");
        local_sprite.pos = match &v["pos"] {
            Value::Object(p) => engine::Position{
                x: match p.get("x").expect("Could not find x position") {
                    Value::Number(px) => px.as_u64().expect("Could not parse x position into a number").try_into().unwrap(),
                    _ => panic!("Did not find a number for the x position"),
                },
                y: match p.get("y").expect("Could not find y position") {
                    Value::Number(py) => py.as_u64().expect("Could not parse y position into a number").try_into().unwrap(),
                    _ => panic!("Did not find a number for the y position"),
                },
            },
            _ => panic!("Did not find a map for the position"),
        };
        local_sprite
    }

    pub fn build_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Could not read file");
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
            "fg": 4,
            "bg": 1,
            "cols": 3,
            "rows": 2,
            "data": [
                "414",
                "114"
            ],
            "pos": {
            	"x": 2,
            	"y": 5
            }
        }
        "#;
        let sp = Sprite::build_from_str(data);
        assert_eq!(sp.pixels.get_fg(), 4);
        assert_eq!(sp.pixels.get_bg(), 1);
        assert_eq!(sp.pixels.get_bits().get_cols(), 3);
        assert_eq!(sp.pixels.get_bits().get_rows(), 2);
        assert_eq!(sp.pixels.get_bits()[engine::Position{x: 0, y: 0}], 4);
        assert_eq!(sp.pixels.get_bits()[engine::Position{x: 1, y: 0}], 1);
        assert_eq!(sp.pixels.get_bits()[engine::Position{x: 2, y: 0}], 4);
        assert_eq!(sp.pixels.get_bits()[engine::Position{x: 0, y: 1}], 1);
        assert_eq!(sp.pixels.get_bits()[engine::Position{x: 1, y: 1}], 1);
        assert_eq!(sp.pixels.get_bits()[engine::Position{x: 2, y: 1}], 4);
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
