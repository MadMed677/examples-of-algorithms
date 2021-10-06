use crate::list::List;
use std::cmp::PartialOrd;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> List<T> for LinkedList<T> {
    fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }
}

impl<T: PartialOrd + Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn from(data: T, child: LinkedList<T>) -> Self {
        Self(Some((data, Box::new(child))))
    }

    pub fn insert_sorted(&mut self, data: T) {
        match self.0 {
            Some((ref value, ref mut child)) => {
                if &data >= value {
                    child.insert_sorted(data);
                } else {
                    self.push_front(data);
                }
            }
            None => self.push_front(data),
        }
    }
}

#[cfg(test)]
mod linked_list {
    use crate::list::List;
    use crate::singly_linked_list::LinkedList;

    #[test]
    fn should_create_linked_list() {
        let mut linked_list = LinkedList::new();
        linked_list.push_front(2);
        linked_list.push_back(1);

        assert_eq!(
            linked_list,
            LinkedList::from(2, LinkedList::from(1, LinkedList::new(),))
        );
    }

    #[test]
    fn should_insert_via_insert_sorted() {
        let mut linked_list = LinkedList::new();
        linked_list.push_front(2);
        linked_list.push_back(4);
        linked_list.push_front(1);
        linked_list.push_back(5);
        linked_list.insert_sorted(3);

        assert_eq!(
            linked_list,
            LinkedList::from(
                1,
                LinkedList::from(
                    2,
                    LinkedList::from(
                        3,
                        LinkedList::from(4, LinkedList::from(5, LinkedList::new()))
                    )
                )
            )
        );
    }

    #[test]
    fn should_insert_all_data_via_insert_sorted() {
        let mut linked_list = LinkedList::new();
        linked_list.insert_sorted(2);
        linked_list.insert_sorted(4);
        linked_list.insert_sorted(1);
        linked_list.insert_sorted(5);
        linked_list.insert_sorted(3);

        assert_eq!(
            linked_list,
            LinkedList::from(
                1,
                LinkedList::from(
                    2,
                    LinkedList::from(
                        3,
                        LinkedList::from(4, LinkedList::from(5, LinkedList::new()))
                    )
                )
            )
        );
    }
}