struct WineShop {
    num_exchange: i32,
    num_empty_bottles: i32,
    num_full_bottles: i32,
}

impl WineShop {
    fn new(num_bottles: i32, num_exchange: i32) -> WineShop {
        WineShop {
            num_full_bottles: num_bottles,
            num_empty_bottles: 0,
            num_exchange,
        }
    }
}

impl Iterator for WineShop {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.num_full_bottles == 0 && self.num_empty_bottles < self.num_exchange {
            return None;
        }

        // Drink
        let drunk_bottles = self.num_full_bottles;
        self.num_empty_bottles += self.num_full_bottles;

        // Exchange
        self.num_full_bottles = self.num_empty_bottles / self.num_exchange;
        self.num_empty_bottles %= self.num_exchange;

        Some(drunk_bottles)
    }
}

struct Solution;
// solution 1: recursive function
// solution 2: impl iterator

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        // Initial
        let mut sum = 0;
        let wineshop = WineShop::new(num_bottles, num_exchange);
        for drunk_bottles in wineshop {
            sum += drunk_bottles;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_0() {
        let res = Solution::num_water_bottles(9, 3);
        assert_eq!(res, 13);
    }

    #[test]
    fn case_1() {
        let res = Solution::num_water_bottles(15, 4);
        assert_eq!(res, 19);
    }

    #[test]
    fn case_2() {
        let res = Solution::num_water_bottles(5, 5);
        assert_eq!(res, 6);
    }

    #[test]
    fn case_3() {
        let res = Solution::num_water_bottles(2, 3);
        assert_eq!(res, 2);
    }

    #[test]
    fn case_4() {
        let res = Solution::num_water_bottles(15, 8);
        assert_eq!(res, 17);
    }
}
