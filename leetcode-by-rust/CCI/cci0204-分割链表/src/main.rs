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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left_head = Box::new(ListNode::new(-1));
        let mut left = &mut left_head;
        let mut right_head = Box::new(ListNode::new(-1));
        let mut right = &mut right_head;
        let mut p = head;

        while let Some(node) = p {
            if node.val < x {
                left.next = Some(Box::new(ListNode::new(node.val)));
                left = left.next.as_mut().unwrap();
            } else {
                right.next = Some(Box::new(ListNode::new(node.val)));
                right = right.next.as_mut().unwrap();
            }

            p = node.next;
        }

        left.next = right_head.next;
        left_head.next
    }
}

fn main() {
    println!("Hello, world!");
}
