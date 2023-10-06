// https://leetcode.com/problems/predict-the-winner/solution/

use std::cmp::max;

/*
#[derive(Debug)]
enum Direction {
    Negative = -1,
    Positive = 1,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Self::Negative => Self::Positive,
            Self::Positive => Self::Negative,
        }
    }
    fn val(&self) -> i32 {
        match self {
            Self::Negative => Self::Negative as i32,
            Self::Positive => Self::Positive as i32,
        }
    }
}
*/

fn scores(nums: &[i32], start: usize, end: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if start == end {
        // 此时只有一种选择，选的人赢对方nums[i]，且没有剩余可选，结束递归
        return nums[start];
    }
    if memo[start][end] != i32::MIN {
        return memo[start][end];
    }
    // 选择左端，获得nums[i]，之后输掉helper(i+1,j)分
    let a = nums[start] - scores(nums, start + 1, end, memo);
    // 选择右端，获得nums[j]，之后输掉helper(i,j-1)分
    let b = nums[end] - scores(nums, start, end - 1, memo);
    // 返回较大者，即在[i,j]数组游戏中胜过对方的分数
    let max_score = max(a, b);
    memo[start][end] = max_score;
    max_score
}

struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let matrix_len = nums.len();
        let mut memo: Vec<Vec<i32>> = vec![vec![i32::MIN; matrix_len]; matrix_len];
        scores(&nums, 0, nums.len() - 1, &mut memo) >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 5, 2];
        let first_win = Solution::predict_the_winner(nums);
        assert!(!first_win);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 5, 233, 7];
        let first_win = Solution::predict_the_winner(nums);
        assert!(first_win);
    }
}
