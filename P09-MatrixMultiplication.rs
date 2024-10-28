use std::ops::{ Index, IndexMut, Mul };
use std::fmt::{ Display, Formatter, Result};
// use std::env;

struct Matrix<T> {
    array2d: Vec<Vec<T>>,
    num_row: usize,
    num_col: usize
}

impl Display for Matrix<i32> {
    fn fmt (&self, f: &mut Formatter) -> Result {
        for i in 0..self.num_row {
            for j in 0..self.num_col {
                let _ = write!(f, "{}\t", self[(i, j)]);
            }
            (|| if i != (self.num_row - 1) {let __ = write!(f, "\n");})();
        }
        return write!(f, "");
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        return &self.array2d[row][col];
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {

    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        return &mut self.array2d[row][col];
    }
}

impl Mul<Matrix<i32>> for Matrix<i32> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if self.num_col != rhs.num_row {
            panic!("Dimension mismatched: cannot multiply the matrices");
        }
        
        let mut result = Matrix::<i32> {
            array2d: vec![vec![0; rhs.num_col]; self.num_row],
            num_row: self.num_row,
            num_col: rhs.num_col
        };

        for i in 0..self.num_row {
            for k in 0..rhs.num_col {
                for j in 0..self.num_col {
                    result[(i, k)] += self[(i, j)] * rhs[(j, k)];
                }
            }
        }
        return result;
    }
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    let matrix = Matrix::<i32> {
        array2d: vec![vec![2, 3, 4], vec![1, 1, 1]],
        num_row: 2,
        num_col: 3
    };

    let other_matrix = Matrix::<i32> {
        array2d: vec![vec![2, 5, 3, 1], vec![1, 2, 3, 1], vec![0, 2, 1, 1]],
        num_row: 3,
        num_col: 4
    };

    let result = matrix * other_matrix;

    println!("{}", result);
}