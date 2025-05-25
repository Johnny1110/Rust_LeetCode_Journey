use std::vec;

use crate::common::Solution;

impl Solution {

    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }

        let mut navigator = MatrixNavigator::new(matrix.len(), matrix[0].len());
        let mut result = Vec::new();

        let mut row = 0;
        let mut col = 0;
        result.push(matrix[row][col].clone());

        for i in 0..matrix.len()*matrix[0].len() - 1 {
            let res = navigator.next(row, col);
            match res {
                Ok((next_row, next_col)) => {
                    row = next_row;
                    col = next_col;
                    result.push(matrix[row][col].clone());
                },
                Err(e) => {
                    break;
                }
            }
        }

        result
    }
}

pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub struct MatrixNavigator {
    direction: Direction,
    top_boundary: (usize, usize),
    bottom_boundary: (usize, usize),
    left_boundary: (usize, usize),
    right_boundary: (usize, usize),
}

impl MatrixNavigator {
    pub fn new(rowLen: usize, colLen: usize) -> Self {
        MatrixNavigator {
             direction: Direction::Right,
            top_boundary: (0, 0), 
            bottom_boundary: (rowLen - 1, colLen - 1), 
            left_boundary: (rowLen - 1, 0), 
            right_boundary: (0, colLen - 1), 
        }
    }

    pub fn next(&mut self, row: usize, col: usize) -> Result<(usize, usize), String> {
        if self.hit_boundary(row, col) {
            self.direction = self.change_direction(row, col)?;
        }

        let next_position = match self.direction {
            Direction::Right => (row, col + 1),
            Direction::Down  => (row + 1, col),
            Direction::Left  => (row, col.saturating_sub(1)),
            Direction::Up    => (row.saturating_sub(1), col),
        };

        Ok(next_position)
    }

    fn change_direction(&mut self, row: usize, col: usize) -> Result<Direction, String> {
        // update current boundaries based on the current direction
        let new_direction = match self.direction {
            Direction::Right => {
                self.top_boundary.0 += 1;
                Direction::Down
            }
            Direction::Down => {
                self.right_boundary.1 -= 1;
                Direction::Left
            }
            Direction::Left => {
                self.bottom_boundary.0 -= 1;
                Direction::Up
            }
            Direction::Up => {
                self.left_boundary.1 += 1;
                Direction::Right
            }
        };

        Ok(new_direction)
    }

    fn hit_boundary(&self, row: usize, col: usize) -> bool {
        match self.direction {
            Direction::Right => col == self.right_boundary.1,
            Direction::Down => row == self.bottom_boundary.0,
            Direction::Left => col == self.left_boundary.1,
            Direction::Up => row == self.top_boundary.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_spiral_order_empty() {
        let matrix: Vec<Vec<i32>> = vec![];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_spiral_order_single_row() {
        let matrix = vec![vec![1, 2, 3]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_spiral_order_1_element() {
        let matrix = vec![vec![1]];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, vec![1]);
    }
}