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

<br>

3. “/”‑diagonals (bottom‑left → top‑right)

Cells share a “/” diagonal when (r + c) is equal; that sum goes from 0 to 2*(n−1):

```rust
let mut diag2_used = vec![false; 2*n - 1];
// index = r + c
```

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

## Let's code


```rust
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        
    }
}
```




