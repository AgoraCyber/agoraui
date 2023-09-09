use crate::{
    framework::FrameworkContext,
    view::{RenderObjectId, View},
};

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

    fn composite_first_render_object_id(
        &self,
        build_context: &FrameworkContext,
    ) -> Option<RenderObjectId> {
        let arena = build_context.element_tree.borrow();
        let children = self.to_id().unwrap().children(&arena);

        for child in children {
            let id = arena.get(child).unwrap().get().to_render_object_id();

            if id.is_some() {
                return id;
            }
        }

        None
    }
}
