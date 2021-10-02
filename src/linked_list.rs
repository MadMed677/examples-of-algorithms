use std::cmp::PartialOrd;
use std::fmt::Display;

#[derive(Debug)]
pub struct LinkedList<T: PartialOrd + Display>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn insert_sorted(&mut self, data: T) {
        match self.0 {
            Some((ref value, ref mut child)) => {
                if &data >= value {
                    println!(
                        "Found the point. The current value is: {} and the next value is: {}",
                        &data, value
                    );
                    child.insert_sorted(data);
                } else {
                    println!(
                        "Didn't find the point. The current value is: {} and the next value is: {}",
                        &data, value
                    );

                    self.push_front(data);
                }
            }
            None => self.push_back(data),
        }
    }
}

#[cfg(test)]
mod linked_list {
    use crate::linked_list::LinkedList;

    #[test]
    fn should_create_linked_list() {
        let mut linked_list = LinkedList::new();
        linked_list.push_front(2);
        linked_list.push_back(1);
    }

    #[test]
    fn should_insert_via_insert_sorted() {
        let mut linked_list = LinkedList::new();
        linked_list.push_front(2);
        linked_list.push_back(4);
        linked_list.push_front(1);
        linked_list.push_back(5);
        linked_list.insert_sorted(3);

        println!("{:?}", linked_list);
    }
}
