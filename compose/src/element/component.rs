use crate::{framework::FrameworkContext, view::View};

use super::{ElementId, Lifecycle};

pub trait ComponentElement: Lifecycle {
    fn build(&self) -> View;

    fn set_child(&self, new: Option<ElementId>);

    fn child(&self) -> Option<ElementId>;

    fn composite_rebuild(&self, build_context: &mut FrameworkContext) {
        let new_configuration = self.build();

        let child = self.child().map(|id| {
            build_context
                .element_tree
                .borrow()
                .get(id)
                .unwrap()
                .get()
                .clone()
        });

        let child = self.update_child(build_context, child, new_configuration);

        self.set_child(child);
    }
}
