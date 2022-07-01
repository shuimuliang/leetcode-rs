use std::cell::{Ref, RefCell};
use std::collections::VecDeque;

struct StockSpanner {
    prices: RefCell<VecDeque<i32>>,
    weights: RefCell<VecDeque<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        StockSpanner {
            prices: RefCell::new(VecDeque::new()),
            weights: RefCell::new(VecDeque::new()),
        }
    }

    fn next(&self, price: i32) -> i32 {
        let mut w: i32 = 1;
        while !self.prices.borrow().is_empty() && *self.prices.borrow().front().unwrap() <= price {
            self.prices.borrow_mut().pop_front();
            w += self.weights.borrow_mut().pop_front().unwrap();
        }

        self.prices.borrow_mut().push_front(price);
        self.weights.borrow_mut().push_front(w);

        w
    }
}

#[cfg(test)]
mod tests {
    use super::StockSpanner;

    #[test]
    fn test_case_1() {
        let obj = StockSpanner::new();
        let ret_1: i32 = obj.next(100);
        assert_eq!(ret_1, 1);
        let ret_1: i32 = obj.next(80);
        assert_eq!(ret_1, 1);
        let ret_1: i32 = obj.next(60);
        assert_eq!(ret_1, 1);
        let ret_1: i32 = obj.next(70);
        assert_eq!(ret_1, 2);
        let ret_1: i32 = obj.next(60);
        assert_eq!(ret_1, 1);
        let ret_1: i32 = obj.next(75);
        assert_eq!(ret_1, 4);
        let ret_1: i32 = obj.next(85);
        assert_eq!(ret_1, 6);
    }
}