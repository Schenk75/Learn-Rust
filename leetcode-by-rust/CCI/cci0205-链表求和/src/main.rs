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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if l1.is_some() && l2.is_some() {
            let mut carry = 0;
            let mut result = Box::new(ListNode::new(-1));
            let mut n = &mut result;
            let mut p = &mut l1.unwrap();
            let mut q = &mut l2.unwrap();

            loop {
                n.next = Some(Box::new(ListNode::new((p.val + q.val + carry) % 10)));
                n = n.next.as_mut().unwrap();
                carry = (p.val + q.val + carry) / 10;

                if p.next.is_none() || q.next.is_none() {break;}
                p = p.next.as_mut().unwrap();
                q = q.next.as_mut().unwrap();
            }

            while p.next.is_some() {
                p = p.next.as_mut().unwrap();
                n.next = Some(Box::new(ListNode::new((p.val + carry) % 10)));
                n = n.next.as_mut().unwrap();
                carry = (p.val + carry) / 10;
            }
            while q.next.is_some() {
                q = q.next.as_mut().unwrap();
                n.next = Some(Box::new(ListNode::new((q.val + carry) % 10)));
                n = n.next.as_mut().unwrap();
                carry = (q.val + carry) / 10;
            }

            if carry != 0 {n.next = Some(Box::new(ListNode::new(carry)));}

            result.next
        } else if l1.is_some() {
            l2
        } else if l2.is_some() {
            l1
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}
