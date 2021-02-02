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
    pub fn get_kth_from_end(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if let None = head {return None;}
        let mut fast = head.as_ref();
        let mut slow = head.as_ref();
        
        for _i in 0..k {
            match fast {
                Some(node) => fast = node.next.as_ref(),
                None => return None
            }
        }

        while let Some(node) = fast {
            slow = slow.unwrap().next.as_ref();
            fast = node.next.as_ref();
        }
        
        Some(slow.unwrap().clone())
    }
}

fn main() {
    println!("Hello, world!");
}
