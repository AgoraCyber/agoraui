//! Tree algorithm structure for Compose crate.

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

pub type Link<T> = Rc<RefCell<Node<T>>>;
pub type WeakLink<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub parent: Option<WeakLink<T>>,
    pub value: T,
    pub children: Vec<Link<T>>,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Node {
            parent: None,
            value,
            children: vec![],
        }
    }
}

impl<T> From<(WeakLink<T>, T)> for Node<T> {
    fn from(value: (WeakLink<T>, T)) -> Self {
        Node {
            parent: Some(value.0),
            value: value.1,
            children: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::Node;

    #[derive(Debug, PartialEq)]
    enum MockType {
        T1,
        T2,
    }

    #[test]
    fn test_from() {
        let link = Rc::new(RefCell::new(MockType::T1.into()));

        let node = Node::<MockType>::from((Rc::downgrade(&link), MockType::T2));

        assert_eq!(node.value, MockType::T2);

        assert_eq!(*node, MockType::T2);
    }
}
