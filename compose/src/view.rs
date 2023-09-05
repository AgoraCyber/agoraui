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

pub trait StatefulConfiguration: IntoView + Debug {
    fn framework_create_state(&self) -> Box<dyn State>;
}

pub trait State {
    fn framework_build(&self, element: &mut StatefulElement) -> View;
}

pub trait StatelessConfiguration: IntoView + Debug {
    fn framework_build(&self, element: &mut StatelessElement) -> View;
}

pub trait RenderObjectConfiguration: IntoView + Debug {
    fn framework_create_render_object(&self) -> Box<dyn RenderObject>;
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

#[derive(Debug, Clone)]
pub enum View {
    Empty,
    Stateful(Configuration<dyn StatefulConfiguration>),
    Stateless(Configuration<dyn StatelessConfiguration>),
    RenderObject(Configuration<dyn RenderObjectConfiguration>),
}

impl View {
    pub fn to_element(self, arena: &mut Arena<Element>) -> Option<ElementId> {
        match self {
            View::Empty => None,
            View::Stateful(config) => StatefulElement::new(arena, config).into(),
            View::Stateless(config) => StatelessElement::new(arena, config).into(),
            View::RenderObject(config) => RenderElement::new(arena, config).into(),
        }
    }
}

impl IntoView for () {
    fn into_view(self) -> View {
        View::Empty
    }
}
