struct Solution;

// solution 1: recursive function
// solution 2: impl iterator

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        // Initial
        let mut sum = 0;
        let mut left_full_bottles = num_bottles;
        let mut left_empty_bottles: i32 = 0;

        loop {
            // Drink
            sum += left_full_bottles;
            left_empty_bottles += left_full_bottles;

            // Exchange
            if left_empty_bottles < num_exchange {
                break;
            }
            left_full_bottles = left_empty_bottles / num_exchange;
            left_empty_bottles = left_empty_bottles % num_exchange;
        }

        sum
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_0() {
        let res = Solution::num_water_bottles(9, 3);
        assert_eq!(res, 13);
    }

    #[test]
    fn case_1() {
        let res = Solution::num_water_bottles(15, 4);
        assert_eq!(res, 19);
    }

    #[test]
    fn case_2() {
        let res = Solution::num_water_bottles(5, 5);
        assert_eq!(res, 6);
    }

    #[test]
    fn case_3() {
        let res = Solution::num_water_bottles(2, 3);
        assert_eq!(res, 2);
    }

    #[test]
    fn case_4() {
        let res = Solution::num_water_bottles(15, 8);
        assert_eq!(res, 17);
    }
}