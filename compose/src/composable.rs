use super::*;

/// Composable view without local state.
pub trait StatelessView {
    fn build(&self, context: &mut dyn BuildContext) -> AnyView;
}

/// Composable view with local state.
pub trait StatefulView {
    /// Create new [ViewState] for this [StatefulView]
    fn create_view_state(&self) -> Box<dyn ViewState>;
}

/// State of [StatefulView]
///
/// When element insert/update, the framework will call
/// [create_view_state](StatefulView::create_view_state) to create new [`State`](ViewState) object
pub trait ViewState {
    fn build(&mut self, context: &mut dyn BuildContext) -> AnyView;
}

#[allow(unused)]
pub trait ViewStateLifecycle {
    fn on_created(&mut self, context: &mut dyn BuildContext) {}

    fn on_mounted(&mut self, context: &mut dyn BuildContext) {}

    fn on_update(&mut self, context: &mut dyn BuildContext) {}

    fn on_unmounted(&mut self, context: &mut dyn BuildContext) {}

    fn on_disposed(&mut self, context: &mut dyn BuildContext) {}
}
