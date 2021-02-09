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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if let Some(mut head) = head {
            let mut slow = &mut head.clone();
            let mut fast = &mut head;

            while fast.next.is_some() {
                fast = fast.next.as_mut().unwrap();
                if fast.next.is_some() {
                    fast = fast.next.as_mut().unwrap();
                    slow = slow.next.as_mut().unwrap();
                } else {
                    break;
                }
            }

            let mut slow = slow.next.take();
            let mut fast = &mut head;

            // 翻转链表
            let mut reverse_res = None;
            while let Some(mut curr) = slow {
                let tmp = curr.next.take();
                curr.next = reverse_res;
                reverse_res = Some(curr);
                slow = tmp;
            }

            while let Some(node) = reverse_res {
                if node.val != fast.val {return false;}
                reverse_res = node.next;
                fast = fast.next.as_mut().unwrap();
            }

            true
        } else {
            true
        }
    }
}

fn main() {
    println!("Hello, world!");
}
