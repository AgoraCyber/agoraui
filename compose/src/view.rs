use super::*;

/// The element configuration object
pub trait View {
    fn into_any_view(self) -> AnyView;
}

pub enum AnyView {
    Stateless(Box<dyn StatelessView>),
    Stateful(Box<dyn StatefulView>),
    Tuple(Box<dyn TupleView>),
}

impl View for AnyView {
    fn into_any_view(self) -> AnyView {
        self
    }
}

pub trait BuildContext {
    fn set_state(&mut self);
}

impl<'a> BuildContext for &'a mut Box<dyn BuildContext> {
    fn set_state(&mut self) {
        self.as_mut().set_state()
    }
}
