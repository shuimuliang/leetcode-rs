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
    use super::{ListNode, reverse_list};

    #[test]
    fn case_1() {
    }

}

