use std::fmt;

// 用链表实现一个链式栈
#[derive(Default, Clone, Debug)]
pub struct ListNode {
    val: String,
    next: Option<Box<ListNode>>,
}

#[derive(Default, Clone, Debug)]
pub struct LinkedListStack {
    node: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: String) -> Self {
        ListNode {val, next: None}
    }
}

impl LinkedListStack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        if self.node.is_none() {true} else {false}
    }

    pub fn push(&mut self, x: String) {
        let mut n = ListNode::new(x);
        n.next = self.node.clone();
        self.node = Some(Box::new(n));
    }

    pub fn pop(&mut self) -> Option<String> {
        if self.is_empty() {return None;}

        let val = self.node.as_ref().unwrap().val.clone();
        self.node = self.node.as_mut().unwrap().next.take();

        Some(val)
    }

    pub fn clear(&mut self) {
        self.node = None;
    }
}

impl fmt::Display for LinkedListStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut current = &self.node;
        let mut result = String::new();
        
        loop {
            match current {
                Some(node) => {
                    if result.is_empty() {
                        result = format!("{}", node.val); 
                    } else {
                       result = format!("{}->{}", result, node.val); 
                    }
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}