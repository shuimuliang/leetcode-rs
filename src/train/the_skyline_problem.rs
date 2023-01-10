// https://leetcode.cn/problems/the-skyline-problem/
// https://desmondwillowbrook.github.io/blog/competitive-programming/dsa-explanations/basic-segment-tree/
// https://cp-algorithms.com/data_structures/segment_tree.html

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn get_skyline(_buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        vec![vec![]]
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[ignore]
    #[test]
    fn case_1() {
        let buildings = vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8],
        ];
        let _res = Solution::get_skyline(buildings);
        // assert_eq!(res, vec![vec![2,10],vec![3,15],vec![7,12],vec![12,0],vec![15,10],vec![20,8],vec![24,0]]);
    }

    #[ignore]
    #[test]
    fn case_2() {
        let buildings = vec![vec![0, 2, 3], vec![2, 5, 3]];
        let _res = Solution::get_skyline(buildings);
        // assert_eq!(res, [vec![0,3],vec![5,0]])
    }
}
