use std::collections::HashMap;

#[allow(dead_code)]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut vecs: Vec<Vec<String>> = Vec::new();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..strs.len() {
        // 将字符串转换为字符数组并对其按字母顺序排序
        let mut chars = vec![];
        for c in strs[i].chars() {
            chars.push(c);
        }
        chars.sort();

        // 将已排序的字符数组转换为字符串
        let key: String = chars.into_iter().collect();

        // 以字母有序的字符串为键在HashMap中进行查找
        let value = map.get(&key);
        if value != None {
            // 找到对应值（字符串动态数组），将原始字符串压入并更新HashMap的键-值对
            let mut v = value.unwrap().to_vec();
            v.push(strs[i].clone());
            map.insert(key, v);
        } else {
            // 未找到对应值，创建以原始字符串初始化的动态数组，并组成键-值对插入HashMap
            let v = vec![strs[i].clone()];
            map.insert(key, v);
        }
    }

    for val in map.values() {
        vecs.push(val.to_vec());
    }

    return vecs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_case_1() {
        let a = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let b = vec![vec!["ate", "eat", "tea"], vec!["nat", "tan"], vec!["bat"]];
        let result = group_anagrams(a);
        assert_eq!(b, result);
    }

    #[test]
    fn test_case_2() {
        let a = vec!["".to_string()];
        let b = vec![vec!["".to_string()]];
        let result = group_anagrams(a);
        assert_eq!(b, result);
    }

    #[test]
    fn test_case_3() {
        let a = vec!["a".to_string()];
        let b = vec![vec!["a".to_string()]];
        let result = group_anagrams(a);
        assert_eq!(b, result);
    }
}
