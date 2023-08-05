use impl_trait_for_tuples::impl_for_tuples;

/// The element configuration object
pub trait View {
    fn into_any_view(self) -> AnyView;
}

pub trait TupleView {}

pub trait ComposableView: View {
    fn build(&mut self, context: &mut Box<dyn BuildContext>) -> AnyView;
}

pub enum AnyView {
    Composable(Box<dyn ComposableView>),
    // Tuple(Box<dyn TupleView>),
    Tuple(Box<dyn TupleView>),
}

impl View for AnyView {
    fn into_any_view(self) -> AnyView {
        self
    }
}

/// [TupleView] support

#[impl_for_tuples(20)]
#[tuple_types_custom_trait_bound(View)]
impl TupleView for Tuple {}

#[impl_for_tuples(20)]
impl View for Tuple {
    for_tuples!( where #( Tuple: View + 'static )* );
    fn into_any_view(self) -> AnyView {
        AnyView::Tuple(Box::new(self))
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
