// https://leetcode-cn.com/problems/compress-string-lcci/

#[allow(dead_code)]
pub fn compress_string(s: String) -> String {
    let mut res = String::new();

    let mut curr_char = '@';
    let mut curr_count = 0;

    let mut chars = s.chars();
    while let Some(t) = chars.next() {
        if curr_char == t {
            curr_count += 1;
        } else {
            if curr_char != '@' {
                res.push(curr_char);
                res.push_str(&curr_count.to_string());
            }

            curr_count = 1;
            curr_char = t;
        }
    }
    res.push(curr_char);
    res.push_str(&curr_count.to_string());
    if res.len() < s.len() {
        return res;
    } else {
        return s;
    }
}

#[cfg(test)]
mod tests {
    use super::compress_string;

    #[test]
    fn case_1() {
        let input_string = String::from("aabcccccaaa");
        let output_string = compress_string(input_string);
        assert_eq!("a2b1c5a3", output_string);
    }

    #[test]
    fn case_2() {
        let input_string = String::from("abbccd");
        let output_string = compress_string(input_string);
        assert_eq!("abbccd", output_string);
    }
}

