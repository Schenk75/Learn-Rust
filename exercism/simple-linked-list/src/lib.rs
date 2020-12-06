use std::{iter::FromIterator, fmt, vec};

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {head: None}
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count: usize = 0;
        let mut current = &self.head;

        loop {
            match current {
                Some(node) => {
                    count += 1;
                    current = &node.next;
                },
                None => break,
            }
        }
        count
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node {data: _element, next: self.head.take()});
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.data)
    }

    pub fn peek(&self) -> Option<&T> {
        let node_data = &self.head.as_ref()?.data;
        Some(node_data)
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut new_list = SimpleLinkedList::new();
        while let Some(val) = self.pop() {
            new_list.push(val);
        }
        new_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut new_list = SimpleLinkedList::new();
        for item in _iter {
            new_list.push(item);
        }
        new_list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut list_rev = self.rev();
        let mut ret = vec![];
        while let Some(val) = list_rev.pop() {
            ret.push(val);
        }

        ret
    }
}

impl<T: fmt::Display> fmt::Display for SimpleLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        let mut result = String::new();
        
        loop {
            match current {
                Some(node) => {
                    if result.is_empty() {
                        result = format!("{}", node.data); 
                    } else {
                       result = format!("{}->{}", result, node.data); 
                    }
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}