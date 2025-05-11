# 52. N-Queens II

<br>

link: https://leetcode.com/problems/n-queens-ii/description/

<br>

---

## Topic 

* Backtracking

<br>

## Brain Strom

Why it seems like same to __051 - n queens__ problem ?

Are there any fast approach to find out how many distinct solutions to the n-queens puzzle ?

Let's revamp N Queens to just return list length as result output, and try that leetcode unit test. and see what's going next.

And then...

![1](imgs/052_1.png)

<br>

Basically, n-queens i and ii is same things, adn let me just get rid of it move to next problem.

<br>

## Let's Coding


```rust
impl Solution {

    pub fn total_n_queens(n: i32) -> i32{
        let mut results = vec![];
        let mut chess_board = ChessBoard::new(n as usize);
        // place is a backtracking function
        Self::place_1(&mut results, &mut chess_board, 0);
        results.len() as i32
    }

    fn place_1(results: &mut Vec<Vec<String>>, chess_board: &mut ChessBoard, row: usize) {
        // if reached the last row, add the current board to results and return
        if row == chess_board.n {
            results.push(chess_board.dump_board_as_result());
            return;
        }

        let n = chess_board.n;

        
        for col in 0..n {

            // check if the current position is safe
            if chess_board.is_safe(row, col) {
                // place the queen
                chess_board.place_queen(row, col);
                // recursively to the next row
                Self::place_1(results, chess_board, row + 1);
                // backtrack and remove the queen
                chess_board.remove_queen(row, col);
            } else {
                if col == n - 1 {
                    return;
                }
                    
            }
        }
    }
}

#[derive(Debug)]
struct ChessBoard {
    pub n: usize,
    pub board: Vec<Vec<bool>>,
    pub safety_mark: SafetyMark,
}

impl ChessBoard {

    pub fn new(n: usize) -> Self {
        let mut board = vec![vec![false; n]; n];
        let mut safety_mark = SafetyMark::new(n);
        Self {
            n,
            board,
            safety_mark,
        }
    }

    pub fn place_queen(&mut self, row: usize, col: usize) {
        self.board[row][col] = true;
        self.safety_mark.mark(row, col);
    }

    pub fn remove_queen(&mut self, row: usize, col: usize) {
        self.board[row][col] = false;
        self.safety_mark.unmark(row, col);
    }

    pub fn is_safe(&self, row: usize, col: usize) -> bool {
        self.safety_mark.is_safe(row, col)
    }

    pub fn dump_board_as_result(&self) -> Vec<String> {
        let mut result = vec![];
        let n = self.n;

        for row in 0..n {
            let mut row_string = String::new();
            for col in 0..n {
                match self.board[row][col] {
                    true => row_string.push('Q'),
                    false => row_string.push('.'),
                }
            }
            result.push(row_string);
        }

        result
    }
}

// define the SafeMark struct
#[derive(Debug)]
struct SafetyMark {
    pub columns: Vec<bool>,
    pub diagonals_top_left_2_bot_right: Vec<bool>,
    pub diagonals_top_right_2_bot_left : Vec<bool>,
}

impl SafetyMark {

    pub fn new(n: usize) -> Self {
        Self {
            columns: vec![false; n],
            diagonals_top_left_2_bot_right: vec![false; 2*n -1],
            diagonals_top_right_2_bot_left: vec![false; 2*n -1],
        }
    }

    pub fn mark(&mut self, row: usize, col: usize) {
        self.columns[col] = true;
        self.diagonals_top_right_2_bot_left[row + col] = true;
        // if x = row-col is negative, x = abs(x) + n-1
        let idx = self.map_row_minus_col_2_index(row, col);
        self.diagonals_top_left_2_bot_right[idx] = true;
    }

    pub fn unmark(&mut self, row: usize, col: usize) {
        self.columns[col] = false;
        self.diagonals_top_right_2_bot_left[row + col] = false;
        
        let idx = self.map_row_minus_col_2_index(row, col);
        self.diagonals_top_left_2_bot_right[idx] = false;
    }

    pub fn is_safe(&self, row: usize, col: usize) -> bool {
        !self.columns[col] &&
        !self.diagonals_top_right_2_bot_left[row + col] &&
        !self.diagonals_top_left_2_bot_right[self.map_row_minus_col_2_index(row, col)]
    }

    // if x = row-col is negative, x = abs(x) + n-1
    fn map_row_minus_col_2_index(&self, row: usize, col: usize) -> usize {
        let mut x = row as i32  - col as i32;
        if x < 0 {
            x = -x + (self.columns.len() - 1) as i32;
        }
        x as usize
    }
}
```