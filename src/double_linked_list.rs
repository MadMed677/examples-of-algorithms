use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DoubleLinkedNode<T> {
    data: T,
    next: Option<Rc<RefCell<DoubleLinkedNode<T>>>>,
    prev: Option<Weak<RefCell<DoubleLinkedNode<T>>>>,
}

#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    first: Option<Rc<RefCell<DoubleLinkedNode<T>>>>,
    last: Option<Weak<RefCell<DoubleLinkedNode<T>>>>,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(current_node) => {
                let new_node = Rc::new(RefCell::new(DoubleLinkedNode {
                    data,
                    next: Some(Rc::clone(&current_node)),
                    prev: None,
                }));

                let mut mutable_current_node = current_node.borrow_mut();
                mutable_current_node.prev = Some(Rc::downgrade(&new_node));

                self.first = Some(new_node);
            }
            None => {
                let new_node = Rc::new(RefCell::new(DoubleLinkedNode {
                    data,
                    next: None,
                    prev: None,
                }));

                self.last = Some(Rc::downgrade(&new_node));
                self.first = Some(new_node);
            }
        }
    }
}

#[cfg(test)]
mod double_linked_list {
    use crate::double_linked_list::DoubleLinkedList;

    #[test]
    fn should_create_double_linked_list() {
        let mut double_linked_list = DoubleLinkedList::new();
        double_linked_list.push_front(6);
        double_linked_list.push_front(4);

        println!("{:?}", double_linked_list);
    }
}
