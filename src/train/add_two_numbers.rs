// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

fn vec_to_list(list: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;

    for x in list.iter().rev() {
        if head.is_none() {
            head = Some(Box::new(ListNode::new(*x)));
        } else {
            let new_node = Box::new(ListNode {
                val: *x,
                next: Some(head.take().unwrap()),
            });
            head = Some(new_node);
        }
    }
    head
}

fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {

    let mut list_iter = head;
    let mut new_vec = Vec::new();

    // 把旧链表拆下，放到新链表头部
    while list_iter.is_some() {
        let mut node = list_iter.as_ref();
        new_vec.push(node.unwrap().val);
        list_iter = &node.unwrap().next;
    }
    new_vec
}

struct Solution;

impl Solution {

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut num_vec : Vec<i32> = Vec::new();
        let mut l1_iter: &Option<Box<ListNode>> = &l1;
        let mut l2_iter: &Option<Box<ListNode>> = &l2;
        let mut plus = 0;

        while l1_iter.is_some() && l2_iter.is_some() {
            let mut l1_node : Option<&Box<ListNode>> = l1_iter.as_ref();
            let v1 = l1_node.unwrap().val;
            l1_iter = &l1_node.unwrap().next;

            let mut l2_node : Option<&Box<ListNode>> = l2_iter.as_ref();
            let v2 = l2_node.unwrap().val;
            l2_iter = &l2_node.unwrap().next;

            let sum = v1 + v2 + plus;
            num_vec.push(sum % 10);
            plus = sum / 10;
        }

        while l1_iter.is_some() {
            let mut node : Option<&Box<ListNode>> = l1_iter.as_ref();
            let v = node.unwrap().val;
            l1_iter = &node.unwrap().next;

            let sum = v + plus;
            num_vec.push(sum % 10);
            plus = sum / 10;
        }

        while l2_iter.is_some() {
            let mut node : Option<&Box<ListNode>> = l2_iter.as_ref();
            let v = node.unwrap().val;
            l2_iter = &node.unwrap().next;

            let sum = v + plus;
            num_vec.push(sum % 10);
            plus = sum / 10;
        }

        // list plus digit
        if plus != 0 {
           num_vec.push(plus);
        }
        vec_to_list(&num_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, vec_to_list, list_to_vec};
    use super::Solution;

    #[test]
    fn test_case_1() {
        // Input: l1 = [2,4,3], l2 = [5,6,4]
        // 2 -> 4 -> 3
        // 5 -> 6 -> 4
        // Output: [7,0,8]
        // Explanation: 342 + 465 = 807.
        let l1 = vec![2,4,3];
        let l2 = vec![5,6,4];
        let n1 = vec_to_list(&l1);
        let n2 = vec_to_list(&l2);
        let res_node = Solution::add_two_numbers(n1, n2);
        let res_vec = list_to_vec(&res_node);
        assert_eq!(vec![7,0,8], res_vec);
    }

    #[test]
    fn test_case_2() {
        // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
        // Output: [8,9,9,9,0,0,0,1]
        //  9999999
        //     9999
        // 10009998
        let l1 = vec![9,9,9,9,9,9,9];
        let l2 = vec![9,9,9,9];
        let n1 = vec_to_list(&l1);
        let n2 = vec_to_list(&l2);
        let res_node = Solution::add_two_numbers(n1, n2);
        let res_vec = list_to_vec(&res_node);
        assert_eq!(vec![8,9,9,9,0,0,0,1], res_vec);
    }
}