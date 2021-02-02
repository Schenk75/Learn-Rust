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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = result;
            result = Some(node);
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
}
