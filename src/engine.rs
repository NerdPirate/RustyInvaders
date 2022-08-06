/********************************************************************
* Copyright (c) 2021-2022, Eric Mackay
* All rights reserved.
*
* This source code is licensed under the BSD-style license found in the
* LICENSE file in the root directory of this source tree.
********************************************************************/

use std::fmt;
use std::fs;
//use std::io::prelude::*;
use std::ops::{Index, IndexMut};
//use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents position on the game board
///
/// 0, 0 are the x, y coordinates indicating the top-leftmost position
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}

/// A simple 2d array
///
/// rows is the number of y indices
/// cols is the number of x indices
#[derive(Debug, Serialize, Deserialize)]
pub struct Array2D<T: Copy> {
    elements: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Copy> Array2D<T> {
    pub fn new(cols: usize, rows: usize, default: (bool, T)) -> Self {
        let mut temp = Self {
            cols: cols,
            rows: rows,
            elements: Vec::<T>::with_capacity(rows * cols),
        };
        if default.0 {
            for _ in 0..rows * cols {
                temp.elements.push(default.1)
            }
        }
        let temp2 = temp;
        temp2
    }

    pub fn in_range(&self, index: &Position) -> bool {
        index.x < self.cols && index.y < self.rows
    }

    pub fn get_elements(&self) -> &Vec<T> {
        &self.elements
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }
}

impl<T: Copy> Clone for Array2D<T> {
    fn clone(&self) -> Self {
        let mut copy = Array2D::<T>::new(self.cols, self.rows, (false, self.elements[0]));
        for i in 0..self.cols * self.rows {
            // TODO Figure out how to enforce self.elements is Vec<T: Copy> and not just Vec<T>
            copy.elements.push(self.elements[i]);
        }
        copy
    }
}

impl<T: fmt::Display + Copy> fmt::Display for Array2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n[ ")?;
        for i in 0..self.rows * self.cols {
            if i != 0 && (i % self.cols) == 0 {
                write!(f, "\n  ")?;
            }
            write!(f, "{} ", self.elements[i])?;
        }
        write!(f, "]")
    }
}

impl<T: Copy> Index<Position> for Array2D<T> {
    type Output = T;
    fn index(&self, index: Position) -> &Self::Output {
        if !self.in_range(&index) {
            panic!("Attempted out-of-range index operation")
        }
        let _real_index = index.y * self.cols + index.x;
        &self.elements[_real_index]
    }
}

impl<T: Copy> IndexMut<Position> for Array2D<T> {
    fn index_mut(&mut self, index: Position) -> &mut <Self as Index<Position>>::Output {
        if !self.in_range(&index) {
            panic!("Attempted out-of-range mutable index operation")
        }
        let _real_index = index.y * self.cols + index.x;
        &mut self.elements[_real_index]
    }
}

/// A simple bitmap using a 2d array
///
/// rows is the number of y indices
/// cols is the number of x indices
#[derive(Debug, Serialize, Deserialize)]
pub struct Bitmap {
    data: Array2D<u8>,
    foreground: u8,
    background: u8,
}

impl Bitmap {
    pub fn new(cols: usize, rows: usize, foreground: u8, background: u8) -> Self {
        Self {
            data: Array2D::<u8>::new(cols, rows, (true, background)),
            foreground: foreground,
            background: background,
        }
    }

    pub fn build_from_str(data: &str) -> Self {
        // TODO Figure out how to error-check that elements = rows * columns
        let bmp: Bitmap = serde_json::from_str(&data).expect("Could not deserialize Bitmap");
        bmp
    }

    pub fn build_from_file(path: &str) -> Self {
        let data = fs::read_to_string(path).expect("Could not read Bitmap file");
        Bitmap::build_from_str(&data)
    }

    pub fn get_data(&self) -> &Array2D<u8> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Array2D<u8> {
        &mut self.data
    }

    pub fn get_fg(&self) -> u8 {
        self.foreground
    }

    pub fn get_fg_mut(&mut self) -> &mut u8 {
        &mut self.foreground
    }

    pub fn get_bg(&self) -> u8 {
        self.background
    }

    pub fn get_bg_mut(&mut self) -> &mut u8 {
        &mut self.background
    }

    // Naive implementation of setting all positions to bg value
    pub fn reset(&mut self) {
        let rows = self.data.get_rows();
        let cols = self.data.get_cols();
        for y_in in 0..rows {
            for x_in in 0..cols {
                self.data[Position { x: x_in, y: y_in }] = self.background
            }
        }
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..(self.data.cols + 2) {
            write!(f, "-")?;
        }
        write!(f, "\n|")?;
        for i in 0..self.data.rows * self.data.cols {
            if i != 0 && (i % self.data.cols) == 0 {
                write!(f, "|\n|")?;
            }
            let bit = &self.data.elements[i];
            if bit == &self.foreground {
                write!(f, "#")?;
            } else if bit == &self.background {
                write!(f, " ")?;
            } else {
                write!(f, "?")?;
            }
        }
        write!(f, "|\n")?;
        for _ in 0..(self.data.cols + 2) {
            write!(f, "-")?;
        }
        write!(f, "\n")?;
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /* Array2D tests */

    #[test]
    fn test_array2d_new_default() {
        let cols = 6;
        let rows = 3;
        let default = 91;
        let array = Array2D::<u8>::new(cols, rows, (true, default));
        for i in 0..cols * rows {
            assert_eq!(array.elements[i], default);
        }
    }

    #[test]
    #[should_panic]
    fn test_array2d_new_nodefault_panic() {
        let cols = 6;
        let rows = 3;
        let default = 91;
        let array = Array2D::<u8>::new(cols, rows, (false, default));
        array.elements[0];
    }

    #[test]
    fn test_array2d_clone() {
        let cols = 6;
        let rows = 3;
        let default = 91;
        let array = Array2D::<u8>::new(cols, rows, (true, default));
        let mut array2 = array.clone();
        array2[Position { x: 4, y: 2 }] = 21;
        assert_eq!(array[Position { x: 4, y: 2 }], default);
        assert_eq!(array2[Position { x: 4, y: 2 }], 21);
    }

    #[test]
    fn test_array2d_index() {
        let mut array = Array2D::<u8>::new(14, 2, (true, 7));
        array.elements[13] = 19;
        array.elements[14] = 4;
        assert_eq!(array[Position { x: 13, y: 0 }], 19);
        assert_eq!(array[Position { x: 0, y: 1 }], 4);
    }

    #[test]
    #[should_panic]
    fn test_array2d_index_panic() {
        let array = Array2D::<u8>::new(14, 2, (true, 7));
        array[Position { x: 14, y: 0 }];
    }

    #[test]
    fn test_array2d_index_mut() {
        let mut array = Array2D::<u8>::new(3, 7, (true, 0));
        array[Position { x: 0, y: 2 }] = 5;
        array[Position { x: 2, y: 0 }] = 1;
        array[Position { x: 2, y: 5 }] = 9;
        assert_eq!(array.elements[0], 0);
        assert_eq!(array.elements[2], 1);
        assert_eq!(array.elements[6], 5);
        assert_eq!(array.elements[17], 9);
        assert_eq!(array.elements[20], 0);
    }

    #[test]
    #[should_panic]
    fn test_array2d_index_mut_panic() {
        let mut array = Array2D::<u8>::new(14, 2, (true, 7));
        array[Position { x: 14, y: 0 }] = 5;
    }

    /* Bitmap Tests */

    #[test]
    fn test_bitmap_new_default() {
        let cols = 6;
        let rows = 3;
        let bg = 0;
        let fg = 1;
        let bitmap = Bitmap::new(cols, rows, fg, bg);
        println!("{}", bitmap);
        /*for i in 0..cols*rows {
            assert_eq!(array.elements[i], default);
        }*/
    }

    #[test]
    fn test_bitmap_build_str() {
        let data = r#"
        {
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
        }
        "#;
        let bm = Bitmap::build_from_str(data);
        assert_eq!(bm.foreground, 4);
        assert_eq!(bm.background, 1);
        assert_eq!(bm.data.cols, 3);
        assert_eq!(bm.data.rows, 2);
        assert_eq!(bm.data[Position { x: 0, y: 0 }], 4);
        assert_eq!(bm.data[Position { x: 1, y: 0 }], 1);
        assert_eq!(bm.data[Position { x: 2, y: 0 }], 4);
        assert_eq!(bm.data[Position { x: 0, y: 1 }], 1);
        assert_eq!(bm.data[Position { x: 1, y: 1 }], 1);
        assert_eq!(bm.data[Position { x: 2, y: 1 }], 4);
    }

    #[test]
    #[should_panic]
    fn test_bitmap_build_str_fail() {
        let data = r#"
        {
            "foreground": 4,
            "data": {
                "rows": 2,
                "cols": 3,
                "elements": [
                    4, 1, 4,
                    1, 1, 4
                ]
            }
        }
        "#;
        // Missing background, so we should fail to build
        let _bm = Bitmap::build_from_str(data);
    }

    #[test]
    fn test_bitmap_reset() {
        let data = r#"
        {
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
        }
        "#;
        let mut bm = Bitmap::build_from_str(data);
        let fg = bm.foreground;
        let bg = bm.background;
        let rows = bm.data.rows;
        let cols = bm.data.cols;
        bm.reset();
        // Make sure size and values not modified
        assert_eq!(bm.foreground, fg);
        assert_eq!(bm.background, bg);
        assert_eq!(bm.data.cols, cols);
        assert_eq!(bm.data.rows, rows);
        // Check everything is set back to bg
        for y_in in 0..rows {
            for x_in in 0..cols {
                assert_eq!(bm.data[Position { x: x_in, y: y_in }], bg);
            }
        }
    }
}
