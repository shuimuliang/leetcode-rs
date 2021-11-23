// pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
//     let a: String = digits.iter()
//         .map(|x| char::from_digit(*x as u32, 10).unwrap())
//         .collect();
//
//     let mut c = a.parse::<i64>().unwrap() + 1;
//
//     let b = c.to_string()
//         .chars()
//         .map(|x| x.to_digit(10).unwrap() as i32)
//         .collect();
//     b
// }


pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut next = 1;
    for i in digits.iter().rev() {
        let pos = *i + next;
        if pos >= 10 {
            v.push(pos % 10);
            next = 1;
        } else {
            v.push(pos);
            next = 0;
        }
    }
    if next != 0 {
        v.push(next);
    }
    v.reverse();
    v
}


#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn test_case_1() {
        let nums = vec![4, 3, 2, 1];
        let result = plus_one(nums);
        assert_eq!(vec![4, 3, 2, 2], result);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let result = plus_one(nums);
        assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1], result);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 6];
        let result = plus_one(nums);
        assert_eq!(vec![7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 7], result);
    }

    #[test]
    fn test_case_4() {
        let nums = vec![9];
        let result = plus_one(nums);
        assert_eq!(vec![1,0], result);
    }
}