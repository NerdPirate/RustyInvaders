
use std::fmt;
use std::ops::{Index, IndexMut};

/// 0, 0 are the x, y coordinates indicating the top-leftmost position
struct Position {
    x: usize,
    y: usize,
}

/// A simple 2d array
///
/// rows is the number of y indices
/// cols is the number of x indices
#[derive(Debug)]
struct Array2D<T> {
    elements: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Copy> Array2D<T> {
    fn new(cols: usize, rows: usize, default: T) -> Self {
        let mut temp = Self {
            cols: cols,
            rows: rows,
            elements: Vec::<T>::with_capacity(rows * cols),
        };
        for _ in 0..rows*cols {
            temp.elements.push(default)
        }
        let temp2 = temp;
        temp2
    }

    fn in_range(&self, index: &Position) -> bool {
        index.x < self.cols && index.y < self.rows
    }
}

impl<T: fmt::Display> fmt::Display for Array2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n[ ")?;
        for i in 0..self.rows*self.cols {
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_array2d_new_default() {
        let cols = 6;
        let rows = 3;
        let default = 91;
        let array = Array2D::<u8>::new(cols, rows, default);
        for i in 0..cols*rows {
            assert_eq!(array.elements[i], default);
        }
    }

    #[test]
    fn test_array2d_index() {
        let mut array = Array2D::<u8>::new(14, 2, 7);
        array.elements[13] = 19;
        array.elements[14] = 4;
        assert_eq!(array[Position{x: 13, y: 0}], 19);
        assert_eq!(array[Position{x: 0, y: 1}], 4);
    }

    #[test]
    #[should_panic]
    fn test_array2d_index_panic() {
        let array = Array2D::<u8>::new(14, 2, 7);
        array[Position{x: 14, y: 0}];
    }

    #[test]
    fn test_array2d_index_mut() {
        let mut array = Array2D::<u8>::new(3, 7, 0);
        array[Position{x: 0, y: 2}] = 5;
        array[Position{x: 2, y: 0}] = 1;
        array[Position{x: 2, y: 5}] = 9;
        assert_eq!(array.elements[0], 0);
        assert_eq!(array.elements[2], 1);
        assert_eq!(array.elements[6], 5);
        assert_eq!(array.elements[17], 9);
        assert_eq!(array.elements[20], 0);
    }

    #[test]
    #[should_panic]
    fn test_array2d_index_mut_panic() {
        let mut array = Array2D::<u8>::new(14, 2, 7);
        array[Position{x: 14, y: 0}] = 5;
    }
}
