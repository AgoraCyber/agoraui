/// The element configuration object
pub trait View {}

pub type AnyView = Box<dyn View>;

pub trait ComposableView {
    fn build(&mut self, context: &ComposableElement) -> AnyView;

    fn set_state(&mut self) {}
}

pub trait BuildContext {}

pub struct ComposableElement {}

impl BuildContext for ComposableElement {}
