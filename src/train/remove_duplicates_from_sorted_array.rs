use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut s = HashSet::new();
    nums.retain(|e| s.insert(*e));
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn test_case_1() {
        let mut v = vec![1, 1, 2];
        let len = remove_duplicates(&mut v);
        assert_eq!(2, len);
        assert_eq!(vec![1,2], v);
    }

    #[test]
    fn test_case_2() {
        let mut v = vec![0,0,1,1,1,2,2,3,3,4];
        let len = remove_duplicates(&mut v);
        assert_eq!(5, len);
        assert_eq!(vec![0,1,2,3,4], v);
    }
}