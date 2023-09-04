use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::RenderObject;

use super::*;

pub trait RenderObjectConfigration: ToElement + IntoView + ToAny + AnyEq + Debug {
    fn create_render_object(&self) -> Box<dyn RenderObject>;
}

pub type RenderObjectView = Configration<dyn RenderObjectConfigration>;

/// Convert [`RenderObjectView`] configuration to [`View`] configuration.
pub fn render_object_to_view<K: Into<KeyPath>, T: RenderObjectConfigration + 'static>(
    keypath: K,
    config: T,
) -> View {
    View::RenderObject(Configration {
        keypath: keypath.into(),
        configration: Rc::new(RefCell::new(config)),
    })
}
