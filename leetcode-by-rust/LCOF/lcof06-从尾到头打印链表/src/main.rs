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

pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut head = head.as_deref();

    while let Some(node) = head {
        result.insert(0, node.val);
        head = node.next.as_deref();

    }
    result
}

fn main() {
    println!("Hello, world!");
}
