/// https://leetcode-cn.com/problems/move-zeroes/

#[allow(dead_code)]
pub fn move_zeroes(nums: &mut Vec<i32>) {
    // 将第i个非0元素，搬到i，剩余的填充0
    let mut _non_zero_count = 0;
    let length = nums.len();

    for i in 0..length {
        // 跳过
        if nums[i] != 0 {
            _non_zero_count += 1;
        }
        // 填充
        else {
            for j in i + 1..length {
                if nums[j] != 0 {
                    nums[i] = nums[j];
                    nums[j] = 0;
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{move_zeroes};

    #[test]
    fn test_case_0() {
        let mut v = vec![2, 1, 5, 3, 12];
        move_zeroes(&mut v);
        assert_eq!(vec![2, 1, 5, 3, 12], v);
    }

    #[test]
    fn test_case_1() {
        let mut v = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut v);
        assert_eq!(vec![1, 3, 12, 0, 0], v);
    }

    #[test]
    fn test_case_2() {
        let mut v = vec![2, 0, 5, 3, 12];
        move_zeroes(&mut v);
        assert_eq!(vec![2, 5, 3, 12, 0], v);
    }
}