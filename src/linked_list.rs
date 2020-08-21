use std::ops::Deref;
use std::mem::swap;

struct Node<T>
where
    T: PartialEq,
{
    pub data: T,
    child: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(data: T) -> Self {
        Self { data, child: None }
    }

    fn append(&mut self, data: T) {
        if let Some(child) = self.child.as_mut() {
            child.append(data);
        } else {
            self.child = Some(Box::new(Node::new(data)));
        }
    }

    fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator::new(self)
    }

    fn insert_after(&mut self, find: &T, data: T) -> Result<(), ()> {
        if &self.data == find {
            let old_child = self.child.take();
            self.child = Some(Box::new(Node {
                data,
                child: old_child,
            }));
            Ok(())
        } else if let Some(child) = self.child.as_mut() {
            child.insert_after(find, data)
        } else {
            Err(())
        }
    }

    fn insert_before(&mut self, find: &T, data: T) -> Result<(), ()> {
        if &self.data == find {
            let mut new_child = Node {
                data,
                child: self.child.take()
            };
            swap(&mut self.data, &mut new_child.data);
            self.child = Some(Box::new(new_child));
            Ok(())
        } else if let Some(child) = self.child.as_mut() {
            child.insert_before(find, data)
        } else {
            Err(())
        }
    }
}

struct LinkedListIterator<'a, T>
where
    T: PartialEq,
{
    current_node: Option<&'a Node<T>>,
}

impl<'a, T> LinkedListIterator<'a, T>
where
    T: PartialEq,
{
    fn new(node: &'a Node<T>) -> Self {
        LinkedListIterator {
            current_node: Some(node),
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T>
where
    T: PartialEq,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.current_node {
            let data = &node.data;
            self.current_node = node.child.as_ref().map(|boxed_item| boxed_item.as_ref());
            Some(data)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new("Test".to_string());
        assert_eq!(node.data, "Test".to_string());
    }

    #[test]
    fn test_append() {
        let mut node = Node::new("Hello".to_string());
        node.append("World".to_string());
        let mut iter = node.iter();
        assert_eq!(iter.next(), Some(&"Hello".to_string()));
        assert_eq!(iter.next(), Some(&"World".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_after() {
        let mut node = Node::new("Hello".to_string());
        node.append("World".to_string());
        assert_eq!(node.insert_after(&"Hello".to_string(), "Beautiful".to_string()), Ok(()));
        let mut iter = node.iter();
        assert_eq!(iter.next(), Some(&"Hello".to_string()));
        assert_eq!(iter.next(), Some(&"Beautiful".to_string()));
        assert_eq!(iter.next(), Some(&"World".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_after_err() {
        let mut node = Node::new("Hello".to_string());
        node.append("World".to_string());
        assert_eq!(node.insert_after(&"Banana".to_string(), "Beautiful".to_string()), Err(()));
    }

    #[test]
    fn test_insert_before() {
        let mut node = Node::new("Hello".to_string());
        node.append("World".to_string());
        assert_eq!(node.insert_before(&"Hello".to_string(), "Well".to_string()), Ok(()));
        let mut iter = node.iter();
        assert_eq!(iter.next(), Some(&"Well".to_string()));
        assert_eq!(iter.next(), Some(&"Hello".to_string()));
        assert_eq!(iter.next(), Some(&"World".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_before_err() {
        let mut node = Node::new("Hello".to_string());
        node.append("World".to_string());
        assert_eq!(node.insert_before(&"Coconut".to_string(), "New".to_string()), Err(()));
    }
}
