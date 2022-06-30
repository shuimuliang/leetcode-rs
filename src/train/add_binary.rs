use std::cmp;

struct Solution;

// impl Solution {
//     pub fn add_binary(a: String, b: String) -> String {
//         let num_a = i64::from_str_radix(&a, 2).unwrap();
//         let num_b = i64::from_str_radix(&b, 2).unwrap();
//         let sum = (num_a + num_b) as i64;
//         let res = format!("{sum:b}");
//         res
//     }
// }

fn char_to_int(c: char) -> usize {
    match c {
        '0' => 0,
        _ => 1,
    }
}

fn int_to_char(i: usize) -> char {
    match i {
        0 => '0',
        _ => '1',
    }
}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let len_a = a.len();
        let len_b = b.len();
        let max_len = cmp::max(a.len(), b.len());
        let mut carry = 0;
        let mut sum = String::new();

        for i in 0..max_len {
            let bit_a = if i < len_a {
                char_to_int(a.chars().nth(len_a - i - 1).unwrap())
            } else { 0 };
            let bit_b = if i < len_b {
                char_to_int(b.chars().nth(len_b - i - 1).unwrap())
            } else { 0 };

            let bit_sum = bit_a + bit_b + carry;
            sum.push(int_to_char(bit_sum % 2));
            if bit_sum > 1 { carry = 1; } else { carry = 0; }
        }
        if carry > 0 {
            sum.push('1');
        }
        sum.chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let a = Solution::add_binary("1010".to_string(), "1011".to_string());
        assert_eq!(a, "10101".to_string());
    }

    #[test]
    fn test_case_2() {
        let a = Solution::add_binary("10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string(),
                                     "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string());
        assert_eq!(a, "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000".to_string());
    }
}
