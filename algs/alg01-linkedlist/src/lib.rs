use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {val, next}
    }
}


#[derive(PartialEq, Eq, Clone)]
pub struct LinkedList {
    head: Option<Box<ListNode>>,
}

impl LinkedList {
    // 新建空链表
    #[allow(dead_code)]
    fn new() -> LinkedList {
        LinkedList {head: None}
    }

    pub fn get_size(&self) -> usize {
        let mut size = 0;
        let mut node = self.head.clone();

        while let Some(box_node) = node {
            size += 1;
            node = box_node.next;
        }
        
        size
    }

    // 判断链表是否为空
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }

    // 向链表头添加元素
    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(ListNode::new(value, self.head.take()));
        self.head = Some(new_node);
    }

    // 从链表头pop元素
    pub fn pop(&mut self) -> Option<i32> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.val)
    }

    // 链表反转
    pub fn reverse(&self) -> LinkedList {
        let mut prev = None;
        let mut curr = self.clone().head;
    
        while let Some(mut node) = curr.take() {
            let next_node = node.next.take();
            node.next = prev.take();
    
            curr = next_node;
            prev = Some(node);
        }
    
        LinkedList {head: prev}
    }

    // 链表的正中间节点
    pub fn middle(&self) -> LinkedList {
        let mut fast_node = &self.head;
        let mut slow_node = &self.head;

        while fast_node.is_some() && fast_node.as_ref().unwrap().next.is_some() {
            slow_node = &slow_node.as_ref().unwrap().next;
            fast_node = &fast_node.as_ref()
                        .unwrap().next.as_ref()
                        .unwrap().next;
        }

        LinkedList {head: slow_node.clone()}
    }

    // 判断链表是否成环
    pub fn is_cycle(&self) -> bool {
        let mut fast_node = &self.head;
        let mut slow_node = &self.head;

        while fast_node.is_some() && fast_node.as_ref().unwrap().next.is_some() {
            slow_node = &slow_node.as_ref().unwrap().next;
            fast_node = &fast_node.as_ref()
                        .unwrap().next.as_ref()
                        .unwrap().next;

            if fast_node == slow_node {return true;}
        }

        false
    }

    // 删除离链表尾第n个节点
    pub fn remove_nth_from_end(&self, n: usize) -> Result<LinkedList, &'static str> {
        let length = self.get_size();
        if n > length {return Err("Out of range");}
        if n <= 0 {return Err("Input should > 0")}

        let mut ret = LinkedList {head: self.head.clone()};
        let idx = length - n - 1;
        let mut curr = ret.head.as_mut();

        for _ in 0..idx {
            curr = curr.unwrap().next.as_mut();
        }
        let next_node = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
        curr.as_mut().unwrap().next = next_node;

        Ok(ret)
    }
}

// println打印链表
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut current = &self.head;
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

// 根据vec生成链表
pub fn make_list(vec: Vec<i32>) -> LinkedList {
    let mut current = None;
    for v in vec.iter().rev() {
        let mut node = ListNode::new(*v, None);
        node.next = current;
        current = Some(Box::new(node));
    }

    LinkedList {head: current}
}

// 合并两个有序链表
pub fn merge_two_sorted_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(node1), None) => Some(node1),
        (None, Some(node2)) => Some(node2),
        (Some(mut node1), Some(mut node2)) => {
            if node1.val > node2.val {
                let n = node2.next.take();
                node2.next = merge_two_sorted_lists(n, Some(node1));
                Some(node2)
            } else {
                let n = node1.next.take();
                node1.next = merge_two_sorted_lists(n, Some(node2));
                Some(node1)
            }

        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{LinkedList, make_list, merge_two_sorted_lists};

    #[test]
    fn make_node_list() {
        let mut list = make_list(vec![1,2,3,4,5]);
        println!("size: {}", list.get_size());
        println!("original list: {}", list);
        list.push(0);
        println!("push item: {}", list);
        list.pop();
        println!("pop item: {}", list);
        println!("reversed list: {}", list.reverse());
        println!("middle: {}", list.middle());

        let i = 2;
        match list.remove_nth_from_end(i) {
            Ok(res) => println!("remove {} item from end of {}: {}", i, list, res),
            Err(err) => println!("{}", err),
        }

        let l1 = make_list(vec![1,3,5,7,9]);
        let l2 = make_list(vec![2,4,6,8,10]);
        println!("merge two lists: {}", LinkedList{head: merge_two_sorted_lists(l1.head, l2.head)});
        
    }
}
