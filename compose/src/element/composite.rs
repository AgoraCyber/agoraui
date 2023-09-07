use super::*;

pub trait CompositeElement: ElementProvider + ToConfiguration + GetChild {
    fn build(&mut self) -> View;

    fn set_child(&mut self, new: Option<ElementId>);

    fn composite_rebuild(&mut self, arena: &mut Arena<Element>) {
        let new_configuration = self.build();

        let child = self
            .child()
            .map(|id| arena.get_mut(id).unwrap().get_mut().clone());

        let child = self.update_child(arena, child, new_configuration);

        self.set_child(child);
    }

    fn update_child(
        &mut self,
        arena: &mut Arena<Element>,
        child: Option<Element>,
        new_configuration: View,
    ) -> Option<ElementId> {
        if let View::Empty = new_configuration {
            if let Some(child) = child {
                self.deactive_child(arena, child.to_id());
            }
            self.set_child(None);
            return None;
        }

        let configuration = self.to_configuration();

        if let Some(child) = child {
            if configuration == new_configuration {
                // Skip update child element.
                Some(child.to_id())
            } else if configuration.same_type(&new_configuration)
                && configuration.to_keypath() == new_configuration.to_keypath()
            {
                // Same element type and path with different configuration.
                child.update_configuration(new_configuration);
                Some(child.to_id())
            } else {
                self.deactive_child(arena, child.to_id());
                self.inflate_view(arena, new_configuration)
            }
        } else {
            self.inflate_view(arena, new_configuration)
        }
    }

    fn deactive_child(&mut self, arena: &mut Arena<Element>, id: ElementId) {
        id.remove(arena);
    }

    fn inflate_view(&mut self, arena: &mut Arena<Element>, configuration: View) -> Option<NodeId> {
        let child_id = configuration.into_element(arena);

        if let Some(child_id) = child_id {
            let element = arena.get(child_id).unwrap().get().clone();

            element.mount(arena, Some(self.to_id()));
        }

        child_id
    }
}
