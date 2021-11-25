#[allow(dead_code)]
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    nums.windows(k as usize).map(|window| *window.iter().max().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let result = max_sliding_window(nums, k);
        assert_eq!(vec![3, 3, 5, 5, 6, 7], result);
    }
}