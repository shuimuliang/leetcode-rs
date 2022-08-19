// https://leetcode.com/problems/predict-the-winner/solution/

use std::cmp::{max, min};

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
        // impl into
        match self {
            Self::Negative => Self::Negative as i32,
            Self::Positive => Self::Positive as i32,
        }
    }
}

fn scores(nums: &[i32], start: usize, end: usize, direction: Direction) -> i32 {
    let score = {
        if start == end {
            direction.val() * nums[start]
        } else {
            match direction {
                Direction::Positive => {
                    let a = nums[start] + scores(nums, start + 1, end, direction.reverse());
                    let b = nums[end] + scores(nums, start, end - 1, direction.reverse());
                    // A try to add max to maximize self scores
                    max(a, b)
                }
                Direction::Negative => {
                    let a = -nums[start] + scores(nums, start + 1, end, direction.reverse());
                    let b = -nums[end] + scores(nums, start, end - 1, direction.reverse());
                    // B try to add min to reduce a's scores
                    min(a, b)
                }
            }
        }
    };
    score
}

struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        scores(&nums, 0, nums.len() - 1, Direction::Positive) >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 5, 2];
        let first_win = Solution::predict_the_winner(nums);
        assert_eq!(first_win, false);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 5, 233, 7];
        let first_win = Solution::predict_the_winner(nums);
        assert_eq!(first_win, true);
    }
}
