use crate::list::List;
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

impl<T> List<T> for DoubleLinkedList<T> {
    fn push_front(&mut self, data: T) {
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

    fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(current_node) => {
                let new_node = Rc::new(RefCell::new(DoubleLinkedNode {
                    data,
                    prev: Some(Weak::clone(&current_node)),
                    next: None,
                }));

                let strong_ref_to_current_node = Weak::upgrade(&current_node).unwrap();
                let mut mutable_current_node = strong_ref_to_current_node.borrow_mut();
                self.last = Some(Rc::downgrade(&new_node));

                mutable_current_node.next = Some(new_node);
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

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            first: None,
            last: None,
        }
    }
}

#[cfg(test)]
mod double_linked_list {
    use crate::doubly_linked_list::DoubleLinkedList;
    use crate::list::List;

    #[test]
    fn should_create_double_linked_list_only_with_push_front() {
        let mut double_linked_list = DoubleLinkedList::new();
        double_linked_list.push_front(6);
        double_linked_list.push_front(4);

        println!("{:?}", double_linked_list);
    }

    #[test]
    fn should_create_double_linked_list_with_push_front_and_push_back() {
        let mut double_linked_list = DoubleLinkedList::new();
        double_linked_list.push_front(2);
        double_linked_list.push_front(1);
        double_linked_list.push_back(3);
        double_linked_list.push_back(4);

        println!("{:?}", double_linked_list);
    }
}
