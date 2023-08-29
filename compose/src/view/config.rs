use crate::KeyPath;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use super::*;

/// Stateless component element configration.
#[derive(Debug)]
pub struct Configration<T: ?Sized> {
    pub(crate) keypath: KeyPath,
    pub(crate) configration: Rc<RefCell<T>>,
}

impl<T> Clone for Configration<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            keypath: self.keypath.clone(),
            configration: self.configration.clone(),
        }
    }
}

impl<T> AnyEq for Configration<T>
where
    T: AnyEq + ?Sized,
{
    fn eq(&self, other: &dyn Any) -> bool {
        self.configration.borrow().eq(other)
    }
}

impl<'a, T> From<&'a Configration<T>> for &'a KeyPath
where
    T: ?Sized,
{
    fn from(value: &'a Configration<T>) -> Self {
        &value.keypath
    }
}
