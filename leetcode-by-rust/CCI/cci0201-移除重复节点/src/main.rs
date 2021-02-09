use std::collections::HashSet;

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
    pub fn remove_duplicate_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut head) = head {
            let mut set = HashSet::new();
            let mut p = &mut head;
            set.insert(p.val);

            while let Some(node) = &p.next {
                if !set.contains(&node.val) {
                    set.insert(node.val);
                    p = p.next.as_mut().unwrap();
                } else {
                    p.next = p.next.take().unwrap().next;
                }
                
            }
            Some(head)
        } else {None}
        
    }
}

fn main() {
    println!("Hello, world!");
}
