// use std::cell::RefCell;
//
// struct MinStack<T> {
//    v: RefCell<Vec<T>>
// }
//
// impl<T> MinStack<T>
// where T: Copy+Clone+Ord+Eq
// {
//     fn new() -> Self {
//        Self {
//           v: RefCell::new(vec![])
//        }
//     }
//
//     fn push(&self, val: T) {
//         self.v.borrow_mut().push(val);
//     }
//
//     fn pop(&self) {
//         self.v.borrow_mut().pop();
//     }
//
//     fn top(&self) -> T {
//         *self.v.borrow().last().unwrap()
//     }
//
//     fn get_min(&self) -> T {
//         *self.v.borrow().iter().min().unwrap()
//     }
// }

use std::cell::RefCell;

#[allow(dead_code)]
pub struct MinStack {
    v: RefCell<Vec<i32>>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        Self {
            v: RefCell::new(vec![]),
        }
    }

    fn push(&self, val: i32) {
        self.v.borrow_mut().push(val);
    }

    fn pop(&self) {
        self.v.borrow_mut().pop();
    }

    fn top(&self) -> i32 {
        *self.v.borrow().last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.v.borrow().iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn test_case_1() {
        let min_stack: MinStack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(-3, min_stack.get_min()); // --> 返回 -3.
        min_stack.pop();
        assert_eq!(0, min_stack.top()); // --> 返回 0.
        assert_eq!(-2, min_stack.get_min()); // --> 返回 -2.
    }
}
