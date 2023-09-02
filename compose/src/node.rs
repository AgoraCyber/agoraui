//! Tree algorithm structure for Compose crate.

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
pub struct Node<V, N> {
    pub parent: Option<Weak<RefCell<N>>>,
    pub value: Rc<RefCell<V>>,
    pub children: Vec<Rc<RefCell<N>>>,
}
