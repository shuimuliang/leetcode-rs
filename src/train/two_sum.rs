// https://leetcode-cn.com/problems/two-sum/

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut value_map:HashMap<i32, Vec<i32>> = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        let index_vec = value_map.get_mut(&value);
        if let Some(v) = index_vec {
            v.push(index as i32);
        }
        else {
            let v = vec![index as i32];
            value_map.insert(*value, v);
        }
    }

    for (index, value) in nums.iter().enumerate() {
        let left = target - (*value as i32);
        if let Some(v) = value_map.get(&left) {
            for i in v {
                if (index as i32) != *i {
                    println!("{} {}", index, i);
                    return vec![index as i32, *i];
                }
            }
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {

    use super::two_sum;

    #[test]
    fn case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(vec![0,1], result);
    }

    #[test]
    fn case_2() {
        let nums = vec![3,3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(vec![0,1], result);
    }
}
