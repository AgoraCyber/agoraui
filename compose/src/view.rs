use std::any::Any;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use indextree::Arena;

use crate::keypath::KeyPath;

pub use super::element::*;
pub use super::render::*;

pub trait IntoView {
    /// Convert [Configration] into [`View`]
    fn into_view(self) -> View;
}

pub trait AnyEq {
    fn eq(&self, rhs: &dyn Any) -> bool;
}

pub trait ToAny {
    fn to_any(&self) -> &dyn Any;
}

pub trait StatefulConfiguration: ToAny + AnyEq + IntoView + Debug {
    fn framework_create_state(&self) -> Box<dyn State>;
}

pub trait State: Debug {
    fn framework_build(&self, element: &StatefulElement) -> View;
}

pub trait StatelessConfiguration: ToAny + AnyEq + IntoView + Debug {
    fn framework_build(&self, element: &StatelessElement) -> View;
}

pub trait RenderObjectConfiguration: ToAny + AnyEq + IntoView + Debug {
    fn framework_render_object_children(&self) -> Vec<View>;
    fn framework_create_render_object(&self) -> RenderObject;
}

#[derive(Debug)]
pub struct Configuration<T: ?Sized> {
    pub key_path: KeyPath,
    pub view: Rc<RefCell<T>>,
}

impl<T> Clone for Configuration<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            key_path: self.key_path.clone(),
            view: self.view.clone(),
        }
    }
}

impl<T> PartialEq for Configuration<T>
where
    T: ?Sized + AnyEq + ToAny,
{
    fn eq(&self, other: &Self) -> bool {
        self.key_path == other.key_path && self.view.borrow().eq(other.view.borrow().to_any())
    }
}

impl<T: StatefulConfiguration + 'static> From<(KeyPath, T)>
    for Configuration<dyn StatefulConfiguration>
{
    fn from(value: (KeyPath, T)) -> Self {
        Configuration {
            key_path: value.0,
            view: Rc::new(RefCell::new(value.1)),
        }
    }
}

impl<T: StatelessConfiguration + 'static> From<(KeyPath, T)>
    for Configuration<dyn StatelessConfiguration>
{
    fn from(value: (KeyPath, T)) -> Self {
        Configuration {
            key_path: value.0,
            view: Rc::new(RefCell::new(value.1)),
        }
    }
}

impl<T: RenderObjectConfiguration + 'static> From<(KeyPath, T)>
    for Configuration<dyn RenderObjectConfiguration>
{
    fn from(value: (KeyPath, T)) -> Self {
        Configuration {
            key_path: value.0,
            view: Rc::new(RefCell::new(value.1)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Empty,
    Stateful(Configuration<dyn StatefulConfiguration>),
    Stateless(Configuration<dyn StatelessConfiguration>),
    RenderObject(Configuration<dyn RenderObjectConfiguration>),
}

impl View {
    pub fn into_element(self, arena: &mut Arena<Element>) -> Option<ElementId> {
        match self {
            View::Empty => None,
            View::Stateful(config) => StatefulElement::new(arena, config).into(),
            View::Stateless(config) => StatelessElement::new(arena, config).into(),
            View::RenderObject(config) => RenderObjectElement::new(arena, config).into(),
        }
    }

    pub fn same_type(&self, view: &View) -> bool {
        match (self, view) {
            (View::Empty, View::Empty) => true,
            (View::Stateful(_), View::Stateful(_)) => true,
            (View::Stateless(_), View::Stateless(_)) => true,
            (View::RenderObject(_), View::RenderObject(_)) => true,
            _ => false,
        }
    }

    pub fn to_keypath(&self) -> Option<&KeyPath> {
        match self {
            View::Empty => None,
            View::Stateful(config) => Some(&config.key_path),
            View::Stateless(config) => Some(&config.key_path),
            View::RenderObject(config) => Some(&config.key_path),
        }
    }
}

impl IntoView for () {
    fn into_view(self) -> View {
        View::Empty
    }
}
