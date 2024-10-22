use std::ops::{ Index, IndexMut, Mul };

struct Matrix<T> {
    array2d: Vec<Vec<T>>,
    num_row: usize,
    num_col: usize
}

// impl<T> Matrix<T> {
//     pub fn multiplication() {
//         let mut result = Matrix::<T> {
//             array2d: vec![vec![22, 32, 42], vec![12, 12, 12]],
//             num_row: 2,
//             num_col: 3
//         };
//     }
// }

// TODO: limit types of T to i and f

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

impl<T> Mul<Matrix<T>> for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        // let mut result = Matrix::<i32> {
        //     array2d: vec![vec![22, 32, 42], vec![12, 12, 12]],
        //     num_row: 2,
        //     num_col: 3
        // };
        println!("inside multipl");
        return self;
    }
}

fn main() {
    let matrix = Matrix::<i32> {
        array2d: vec![vec![2, 3, 4], vec![1, 1, 1]],
        num_row: 2,
        num_col: 3
    };

    let other_matrix = Matrix::<i32> {
        array2d: vec![vec![2, 3, 4], vec![1, 1, 1]],
        num_row: 2,
        num_col: 3
    };

    let result = matrix * other_matrix;

    println!("{}", result[(0, 1)]);
}