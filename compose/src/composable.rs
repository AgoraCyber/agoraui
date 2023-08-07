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
pub trait ViewState {}
