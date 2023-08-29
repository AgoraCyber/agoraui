mod stateless;
pub use stateless::*;

mod stateful;
pub use stateful::*;

mod config;
pub use config::*;

use crate::{Element, KeyPath};
use std::any::Any;

/// Render element configration trait.
pub trait IntoView {
    /// Convert [Configration] into [`View`]
    fn into_view(self) -> View;
}

/// Convert configration to any
pub trait ToAny {
    /// Convert Self to [`Any`]
    fn to_any(&self) -> &dyn Any;
}

/// Eq trait to other [`Any`] object
pub trait AnyEq {
    fn eq(&self, other: &dyn Any) -> bool;
}

pub trait ToElement {
    fn to_element(&self, view: View) -> Element;
}

/// UI render element configuration.
#[derive(Debug, Clone)]
pub enum View {
    /// Empty configuration will generate None render element
    Empty,
    /// Stateless component ui configration.
    Stateless(Stateless),
    /// Stateful compoent ui configration.
    Stateful(Stateful),
}

impl IntoView for () {
    fn into_view(self) -> View {
        View::Empty
    }
}

impl PartialEq for View {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Stateless(l0), Self::Stateless(r0)) => l0.eq(r0.configration.borrow().to_any()),
            (Self::Stateful(l0), Self::Stateful(r0)) => l0.eq(r0.configration.borrow().to_any()),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl IntoView for View {
    fn into_view(self) -> View {
        // [`View`] simply returns self
        self
    }
}

impl AnyEq for View {
    fn eq(&self, other: &dyn Any) -> bool {
        match self {
            View::Empty => true,
            View::Stateless(config) => config.eq(other),
            View::Stateful(config) => config.eq(other),
        }
    }
}

impl<'a> From<&'a View> for &'a KeyPath {
    fn from(value: &'a View) -> Self {
        match value {
            View::Empty => &KeyPath::Empty,
            View::Stateless(config) => config.into(),
            View::Stateful(config) => config.into(),
        }
    }
}

impl View {
    pub fn to_key_path(&self) -> &KeyPath {
        self.into()
    }

    pub fn to_element(&self) -> Element {
        match self {
            View::Empty => Element::Empty,
            View::Stateless(config) => config.configration.borrow().to_element(self.clone()),
            View::Stateful(config) => config.configration.borrow().to_element(self.clone()),
        }
    }
}
