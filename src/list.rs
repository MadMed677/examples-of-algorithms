/// Base trait for any type of Lists
/// As an example:
///  - [Singly linked List](https://en.wikipedia.org/wiki/Linked_list#Singly_linked_list)
///  - [Doubly linked List](https://en.wikipedia.org/wiki/Linked_list#Doubly_linked_list)
///  - [Multiply linked List](https://en.wikipedia.org/wiki/Linked_list#Multiply_linked_list)
///  and so on
pub trait List<T> {
    /// Command which add the `data` into the **front** of the list
    fn push_front(&mut self, data: T);

    /// Command which add the `data` into the **back** of the list
    fn push_back(&mut self, data: T);
}
