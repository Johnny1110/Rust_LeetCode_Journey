use crate::common::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let the_end_position = (nums.len() as i32) - 1;
        let mut total_jump_count = 0;
        let mut current_position = 0;


        while current_position < the_end_position {

            // find the farthest position we can reach
            let possible_jump_step = nums[current_position as usize];
            let mut farthest_position = current_position + possible_jump_step;

            let mut expect_next_round_idx_temp = 0;

            for i in 1..possible_jump_step+1 {

                let abs_jump_position = current_position + i;


                if abs_jump_position >= the_end_position {
                    farthest_position = abs_jump_position;
                    break;
                }

                // find the best value in the possible jump
                let expect_next_round_idx = abs_jump_position + nums[abs_jump_position as usize];
                if (expect_next_round_idx > expect_next_round_idx_temp) {
                    farthest_position = abs_jump_position;
                    expect_next_round_idx_temp = expect_next_round_idx;
                }

            }
            
            current_position = farthest_position;
            total_jump_count += 1;
        }
        
        return total_jump_count;
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::jump(vec![2,1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::jump(vec![2, 3, 1]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::jump(vec![10,9,8,7,6,5,4,3,2,1,1,0]), 2);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::jump(vec![4,1,1,3,1,1,1]), 2);
    }
}

