use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct LinkedList<T: PartialEq>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialEq> LinkedList<T> {
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
}

#[cfg(test)]
mod linked_list {
    use crate::linked_list::LinkedList;

    #[test]
    fn it_works() {
        let mut linked_list = LinkedList::new();
        linked_list.push_front(2);
        linked_list.push_back(1);

        println!("{:?}", linked_list);

        assert_eq!(2 + 2, 4);
    }
}
