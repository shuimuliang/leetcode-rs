use std::collections::HashMap;

#[allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();
    s.chars().for_each(|c| {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });
    t.chars().for_each(|c| {
        map.entry(c).and_modify(|v| *v -= 1).or_insert(-1);
    });
    map.iter().all(|(_, v)| *v == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let result = is_anagram(s, t);
        assert_eq!(true, result);
    }

    #[test]
    fn test_case_2() {
        let s = "rat".to_string();
        let t = "car".to_string();
        let result = is_anagram(s, t);
        assert_eq!(false, result);
    }
}