use std::{marker::PhantomData, ptr::NonNull};

pub struct Node<T> {
    pub val: T,
    pub next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub len: usize,
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    pub fn insert_at_head(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = self.head;
        node.prev = None;

        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
        }

        self.head = node_ptr;
        self.len += 1;
    }

    pub fn insert_at_tail(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.tail;

        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
        }
    }

    pub fn delete_head(&mut self) {
        if self.len == 0 {
            return None;
        }

        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
        });
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn insert_at_head_works() {
        let mut list = LinkedList::<i32>::new();
        let second_val = 2;
        list.insert_at_head(1);
        list.insert_at_head(second_val);
        println!("Linked List is {:?}", list);
    }
}
