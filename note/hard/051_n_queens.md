# 051 - N Queens

<br>

---

<br>

[Back](../..)

## Problem Link
https://leetcode.com/problems/n-queens/description/

<br>

## Topic

* Array
* Backtracking (QAQ dam.. that's so tough)

<br>

## Brain Storm

That one is really toughm and also I can't really handle backtracking problem very well. 

I want to figure out the problem first. and then ask chat-GPT for some advise.

<br>

### How do queens move in chess?

The queen can move any number of squares horizontally, vertically, or diagonally in a straight line.

<br>
<br>

## We have `n` queens and `n x n` chessboard.

1. Think row by row

    We need to place exactly one queen in each row (since two quuen in same row will attack each other).

    __Q: If we on row  `r`, what can we do?__

<br>

2. Checking for safety

    When we put a queen on the chessboard, She gonna be attack by others if she stand on outher queens AOE range (horizontally, vertically, or diagonally).

    __Q: We don't have to care about the row occupied, cuz 1 for loop chessborad rows, 1 row only put 1 queen. But how do we know which columns are already occupied?__

    __Q: What's a simple way to check diagonals safety?__

    <br>

    So, the biggest problem is how do we trace column `c` & `diagonals` is safe.

<br>

3. Backtracking scaffold

    How do we desgin the backtracking func?

    Here's the func `place(currentRow)`:

    1. if `currentRow == n` that means we already placed all n queens, record current chessboard as a solution backtrack.

    2. Otherwise, for each column `c` (0 ~ n-1):

        * Check row is safe or not.

        * If safe:

            * `Mark` column and diagonals are in used.

            * Recurse `place(currentRow + 1)`

            * `Unmark` and try next `c` (backtrack step)

    <br>

    __Q: What data struct can let us `Mark` and  `unmark` quickly?__

    <br>

4. When we placed all queens, We have to turn our chessboard into a tring/array format.

    __Q: How could we represent out partial solution as we build it.__

<br>
<br>
<br>
<br>

### Talk to Chat-GPT

__Ask__

I think the biggest problem is how to design a data struct to implement `isSafe()` func, which means we can mark which columns already taken and also left and right diagonals is safe ot not.

1. mark column:

    I think we can just using a usize arraylist.

2. mark left diagonals:

    I think we can using a `n x n` bitmap maybe? I'm not sure.

3. mark right diagonals:

    `n x n` bitmap also.

Am I right?

<br>
<br>

__Answer__

You’re on the right track in wanting a constant‑time check instead of scanning the whole board each time, but you don’t need an n×n bitmap for the diagonals—just two 1D arrays (or bit‑fields) of length 2n−1 each. Here’s how you can set it up:

1. Columns
Keep a boolean array (or Vec<bool>) of size n:

```rust
let mut col_used = vec![false; n];
```

When you place at (r,c), set col_used[c] = true; when you remove, set it back to false.

<br>

2. “\”‑diagonals (top‑left → bottom‑right)

Any cell (r,c) lies on the same “\” diagonal as another if (r − c) is equal. But (r−c) ranges from −(n−1) to +(n−1), so shift by +(n−1) to index a 0‑based array of length 2*n−1:

```rust
let mut diag1_used = vec![false; 2*n - 1];
// index = (r - c) + (n - 1)
```

For examle, if `n=4`:

* The max `row-col` is `3-0` equals to 3.
* The min `row-col` is `0-3` equals to -3.

So all the possiable `row-col` nums is `[-3, -2, -1, 0, 1, 2, 3]` the length is 7.

How do we map `row-col` to index? 

If `row-col` is negative then abs it and plus `n-1`. Like `row-col = -3` then `abs(-3) + 3` and we got index 6.


<br>

3. “/”‑diagonals (bottom‑left → top‑right)

Cells share a “/” diagonal when (r + c) is equal; that sum goes from 0 to 2*(n−1):

```rust
let mut diag2_used = vec![false; 2*n - 1];
// index = r + c
```

For examle, if `n=4`:

* The max `row+col` is `0+0` equals to 0.
* The min `row+col` is `3+3` equals to 6.

So all the possiable `row-col` nums is `[0, 1, 2, 3, 4, 5, 6]` the length is 7.

<br>

__Putting it together in `is_safe(r, c)`__

```rust
fn is_safe(r: usize, c: usize, 
           col: &[bool], diag1: &[bool], diag2: &[bool], n: usize) -> bool {
    let d1 = (r as isize - c as isize + (n-1) as isize) as usize;
    let d2 = r + c;
    !col[c] && !diag1[d1] && !diag2[d2]
}
```

<br>

And in your backtracking:

```rust
// place queen
col[c]      = true;
diag1[d1]   = true;
diag2[d2]   = true;

// recurse to next row…

// remove queen (backtrack)
col[c]      = false;
diag1[d1]   = false;
diag2[d2]   = false;
```

<br>
<br>
<br>
<br>

## Let's Coding


```rust
impl Solution {

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut results = vec![];
        let mut chess_board = ChessBoard::new(n as usize);
        // place is a backtracking function
        Self::place(&mut results, &mut chess_board, 0);
        results
    }

    fn place(results: &mut Vec<Vec<String>>, chess_board: &mut ChessBoard, row: usize) {
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
                Self::place(results, chess_board, row + 1);
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
pub struct ChessBoard {
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
pub struct SafetyMark {
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

<br>

Perfect !

![1](imgs/051_1.png)




