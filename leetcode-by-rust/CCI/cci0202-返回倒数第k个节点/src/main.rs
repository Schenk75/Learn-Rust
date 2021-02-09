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
    pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
        if head.is_none() {return -1;}
        let mut slow = head.clone();
        let mut fast = head.clone();

        for _i in 0..k-1 {
            if let Some(node) = fast.unwrap().next {
                fast = Some(node);
            } else {
                return -1;
            }
        }

        while let Some(node) = fast.unwrap().next {
            fast = Some(node);
            slow = slow.unwrap().next;
        }

        slow.unwrap().val
    }
}

fn main() {
    println!("Hello, world!");
}
