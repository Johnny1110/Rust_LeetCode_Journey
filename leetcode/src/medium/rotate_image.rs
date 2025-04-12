use core::num;

use crate::common::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let side_length = matrix.len() as usize;

        // 1. do flip horizontally.
        Self::flip_horizontally(matrix, side_length);
        // 2. do flip diagonally.
        Self::flip_diagonally(matrix, side_length);
    }

    // How to flip horizontally?
    // Swap Matrix(row, col) to Matrix(n-1-row, col)
    fn flip_horizontally(matrix: &mut Vec<Vec<i32>>, side_length: usize) {
        for i in 0..side_length / 2 {
            for j in 0..side_length {
                let temp = matrix[i as usize][j as usize];
                matrix[i as usize][j as usize] = matrix[side_length - 1 - i as usize][j as usize];
                matrix[side_length - 1 - i as usize][j as usize] = temp;
            }
        }
    }

    // How to flip diagonally?
    // Swap Matrix(row, col) to Matrix(col, row)
    fn flip_diagonally(matrix: &mut Vec<Vec<i32>>, side_length: usize) {
        for i in 0..side_length {
            for j in i..side_length {
                let temp = matrix[i as usize][j as usize];
                matrix[i as usize][j as usize] = matrix[j as usize][i as usize];
                matrix[j as usize][i as usize] = temp;
            }
        }

    }
}

// -----------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7,4,1], vec![8,5,2], vec![9,6,3]]);
    }

    #[test]
    fn test_2() {
        let mut matrix = vec![vec![5,1,9,11], vec![2,4,8,10], vec![13,3,6,7], vec![15,14,12,16]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![15,13,2,5], vec![14,3,4,1], vec![12,6,8,9], vec![16,7,10,11]]);
    }
}