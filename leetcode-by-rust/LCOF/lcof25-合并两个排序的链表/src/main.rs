// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut node = head.as_mut();

        while l1.is_some() && l2.is_some() {
            let curr;
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let next = l1.as_mut().unwrap().next.take();
                curr = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                curr = l2;
                l2 = next;
            }
            node.as_mut().unwrap().next = curr;
            node = node.unwrap().next.as_mut();
        }

        node.as_mut().unwrap().next = if l1.is_some() {l1.take()} else {l2.take()};       
        head.unwrap().next
    }
}

fn main() {
    println!("Hello, world!");
}
