// Definition for singly-linked list.
// 给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。
// https://leetcode-cn.com/problems/reverse-linked-list/

#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(dead_code)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut old_list = head;
    let mut new_list = Option::None;

    // 把旧链表拆下，放到新链表头部
    while old_list.is_some() {
        let mut node = old_list.unwrap();
        let rem = node.next.take();
        node.next = new_list;
        new_list = Some(node);
        old_list = rem;
    }
    new_list
}

#[cfg(test)]
mod tests {
    use super::{reverse_list, ListNode};

    #[test]
    fn case_1() {
        // 1->2->3->4->5->NULL
        let l5 = Box::new(ListNode::new(5));
        let mut l4 = Box::new(ListNode::new(4));
        let mut l3 = Box::new(ListNode::new(3));
        let mut l2 = Box::new(ListNode::new(2));
        let mut l1 = Box::new(ListNode::new(1));
        l4.next = Some(l5);
        l3.next = Some(l4);
        l2.next = Some(l3);
        l1.next = Some(l2);

        let head = reverse_list(Some(l1));
        let rev_l1 = head.unwrap();
        assert_eq!(5, rev_l1.val);
        let rev_l2 = rev_l1.next.unwrap();
        assert_eq!(4, rev_l2.val);
        let rev_l3 = rev_l2.next.unwrap();
        assert_eq!(3, rev_l3.val);
        let rev_l4 = rev_l3.next.unwrap();
        assert_eq!(2, rev_l4.val);
        let rev_l5 = rev_l4.next.unwrap();
        assert_eq!(1, rev_l5.val);
        println!("{:?}", rev_l5.next);
        let tail = rev_l5.next;
        assert!(tail.is_none());
    }
}
