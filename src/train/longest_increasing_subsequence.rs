// https://leetcode-cn.com/problems/longest-increasing-subsequence/

#[allow(dead_code)]
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut b_array: Vec<i32> = Vec::with_capacity(nums.len());

    for value in &nums {
        let pos_res = b_array.binary_search(value);

        match pos_res {
            // 匹配到, 重复元素不做任何处理
            Ok(_) => {}

            // 未匹配到，根据位置判定
            Err(pos) => {
                if (pos as i32) == 0 {
                    // 空数组，放第一个
                    if b_array.is_empty() {
                        b_array.push(*value);
                    } else {
                        // 非空数组，替换第一个
                        if *value < b_array[0] {
                            b_array[0] = *value;
                        }
                    }
                } else if pos < b_array.len() {
                    b_array[pos] = *value;
                } else {
                    b_array.push(*value);
                }
            }
        }
        println!("after {:?}", b_array);
    }
    b_array.len() as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_lis;

    #[test]
    fn case_1() {
        let input = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let output = length_of_lis(input);
        assert_eq!(4, output);
    }
}
