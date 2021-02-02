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
    pub fn delete_node(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        // 双指针，添加一个头节点
        let mut head = Some(Box::new(ListNode {val: 0, next: head}));
        let mut p = &mut head;

        while let Some(node) = p {
            let next_node = &mut node.next;
            match next_node {
                None => break,
                Some(next) => {
                    // 当前节点的下一个节点的值等于val
                    if next.val == val {
                        node.next = next.next.take();
                        break;
                    }
                }
            }
            p = &mut node.next;
        }

        head.unwrap().next
    }
}

fn main() {
    println!("Hello, world!");
}
