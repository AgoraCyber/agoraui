use impl_trait_for_tuples::impl_for_tuples;

use super::element::*;

/// The element configuration object
pub trait View {
    fn into_any_view(self) -> AnyView;

    fn create_element<'a>(&'a self) -> Box<dyn Elemement<'a>>;
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

    fn create_element<'a>(&'a self) -> Box<dyn Elemement<'a>> {
        match self {
            AnyView::Stateful(view) => {
                return view.create_element();
            }
            AnyView::Stateless(view) => {
                return view.create_element();
            }
            AnyView::Tuple(view) => {
                return view.create_element();
            }
        }
    }
}
/// Composite view build context .
pub trait BuildContext {
    fn set_state(&mut self);
}

impl<'a> BuildContext for &'a mut Box<dyn BuildContext> {
    fn set_state(&mut self) {
        self.as_mut().set_state()
    }
}

/// Composable view without local state.
pub trait StatelessView: View {
    fn framework_build(&self, context: &mut dyn BuildContext) -> AnyView;
}

/// Composable view with local state.
pub trait StatefulView: View {
    /// Create new [ViewState] for this [StatefulView]
    fn framework_create_view_state(&self) -> Box<dyn ViewState>;
}

/// State of [StatefulView]
///
/// When element insert/update, the framework will call
/// [create_view_state](StatefulView::create_view_state) to create new [`State`](ViewState) object
pub trait ViewState: ViewStateLifecycle {
    fn framework_build(&mut self, context: &mut dyn BuildContext) -> AnyView;
}

#[allow(unused)]
pub trait ViewStateLifecycle {
    fn on_created(&mut self, context: &mut dyn BuildContext) {}

    fn on_mounted(&mut self, context: &mut dyn BuildContext) {}

    fn on_update(&mut self, context: &mut dyn BuildContext) {}

    fn on_unmounted(&mut self, context: &mut dyn BuildContext) {}

    fn on_disposed(&mut self, context: &mut dyn BuildContext) {}
}

pub trait TupleView: View {}

/// [TupleView] support
#[impl_for_tuples(20)]
#[tuple_types_custom_trait_bound(View + 'static)]
impl TupleView for Tuple {}

#[impl_for_tuples(20)]
impl View for Tuple {
    for_tuples!( where #( Tuple: View + 'static )* );
    fn into_any_view(self) -> AnyView {
        AnyView::Tuple(Box::new(self))
    }

    fn create_element<'a>(&'a self) -> Box<dyn Elemement<'a>> {
        unimplemented!()
    }
}
