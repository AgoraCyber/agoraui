use std::fmt::Debug;

use super::*;

pub trait RenderObjectConfigration: ToElement + IntoView + ToAny + AnyEq + Debug {
    fn create_render_object(&self);
}
