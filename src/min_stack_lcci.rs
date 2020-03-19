// https://leetcode-cn.com/problems/min-stack-lcci/

use std::cell::RefCell;

struct MinStack {
    sim_stack: RefCell<Vec<i32>>,
    min_stack: RefCell<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            sim_stack: RefCell::new(vec![]),
            min_stack: RefCell::new(vec![]),
        }
    }

    fn push(&self, x: i32) {
        let mut sim_stack = self.sim_stack.borrow_mut();
        sim_stack.push(x);

        let mut min_stack =self.min_stack.borrow_mut();
        let top_value = min_stack.last().cloned();

        if let Some(top_v) = top_value {
            if x < top_v{
                min_stack.push(x);
            } else {
                min_stack.push(top_v);
            }
        } else {
            min_stack.push(x);
        }
    }

    fn pop(&self) {
        let mut sim_stack= self.sim_stack.borrow_mut();
        sim_stack.pop();
        let mut min_stack = self.min_stack.borrow_mut();
        min_stack.pop();
    }

    fn top(&self) -> i32 {
        let sim_stack= self.sim_stack.borrow();
        *sim_stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        let min_stack = self.min_stack.borrow();
        *min_stack.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {

    use super::MinStack;

    #[test]
    fn case_1() {

        let min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        let ret_1 = min_stack.get_min();
        assert_eq!(-3, ret_1);

        min_stack.pop();
        let ret_2 = min_stack.top();
        assert_eq!(0, ret_2);

        let ret_3 = min_stack.get_min();
        assert_eq!(-2, ret_3);

    }
}