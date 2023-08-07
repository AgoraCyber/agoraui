use impl_trait_for_tuples::impl_for_tuples;

use super::view::*;

pub trait TupleView {}

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
