// https://leetcode-cn.com/problems/valid-parentheses/

use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let symbol_tuples: Vec<(char, char)> = vec![('(', ')'), ('[', ']'), ('{', '}')];
    let symbol_map: HashMap<char, char> = symbol_tuples.into_iter().collect();

    let mut v = vec![];
    let mut iter = s.chars();
    while let Some(c) = iter.next() {
        let _is_continue = match c {
            '(' | '{' | '[' => {
                v.push(c);
                true
            }
            ')' | ']' | '}' if v.last().is_some() && (symbol_map.get(&v.last().unwrap()) == Some(&c)) => {
                v.pop();
                true
            }
            _ => false,
        };
        if !_is_continue {
            return false;
        }
    }
    v.is_empty()
}


#[cfg(test)]
mod tests {
    use super::{is_valid};

    #[test]
    fn test_case_0() {
        let s = "()[]{}".to_string();
        let v = is_valid(s);
        assert_eq!(true, v);
    }

    #[test]
    fn test_case_1() {
        let s = "(]".to_string();
        let v = is_valid(s);
        assert_eq!(false, v);
    }

    #[test]
    fn test_case_3() {
        let s = "([)]".to_string();
        let v = is_valid(s);
        assert_eq!(false, v);
    }

    #[test]
    fn test_case_4() {
        let s = "{[]}".to_string();
        let v = is_valid(s);
        assert_eq!(true, v);
    }

    #[test]
    fn test_case_5() {
        let s = "]".to_string();
        let v = is_valid(s);
        assert_eq!(false, v);
    }
}