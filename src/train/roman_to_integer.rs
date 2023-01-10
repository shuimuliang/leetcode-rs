struct Solution;

fn char_to_int(s: char) -> i32 {
    match s {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut previous: i32 = 0;
        for (index, x) in s.chars().enumerate() {
            let map_value = char_to_int(x);

            if index == 0 {
                sum += map_value;
            } else {
                let map_value = char_to_int(x);
                if previous < map_value {
                    sum -= previous;
                    sum += map_value - previous;
                } else {
                    sum += map_value;
                }
            }

            previous = map_value;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_0() {
        let s: String = "I".into();
        let res = Solution::roman_to_int(s);
        assert_eq!(res, 1);
    }

    #[test]
    fn case_1() {
        let s: String = "III".into();
        let res = Solution::roman_to_int(s);
        assert_eq!(res, 3);
    }

    #[test]
    fn case_2() {
        let s: String = "LVIII".into();
        let res = Solution::roman_to_int(s);
        assert_eq!(res, 58);
    }

    #[test]
    fn case_3() {
        let s: String = "MCMXCIV".into();
        let res = Solution::roman_to_int(s);
        assert_eq!(res, 1994);
    }

    #[test]
    fn case_4() {
        let s: String = "MMMCMXCIX".into();
        let res = Solution::roman_to_int(s);
        assert_eq!(res, 3999);
    }
}
