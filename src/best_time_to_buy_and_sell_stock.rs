// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/

use std::cmp::{max, min};

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_rtn= 0;
    let mut min_price = 0;
    for (index, value) in prices.iter().enumerate() {
        if index == 0 {
            max_rtn = 0;
            min_price = *value;
        } else {
            // 和前一天比较
            max_rtn = max(max_rtn, *value-min_price);
            min_price = min(min_price, *value);
        }
    }
    max_rtn
}

#[cfg(test)]
mod tests {

    use super::max_profit;

    #[test]
    fn case_1() {
        let input_vec = vec![7,1,5,3,6,4];
        let output = max_profit(input_vec);
        assert_eq!(5, output);
    }

    #[test]
    fn case_2() {
        let input_vec = vec![7,6,4,3,1];
        let output = max_profit(input_vec);
        assert_eq!(0, output);
    }
}
