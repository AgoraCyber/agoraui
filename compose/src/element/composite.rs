use super::*;

pub trait CompositeElement: ElementProvider + ToConfiguration {
    fn build(&mut self) -> View;

    fn child(&self) -> Option<ElementId>;

    fn set_child(&mut self, new: Option<ElementId>);

    fn rebuild(&mut self, arena: &mut Arena<Element>) {
        let new_configuration = self.build();

        self.update_child(arena, new_configuration);
    }

    fn update_child(&mut self, arena: &mut Arena<Element>, new_configuration: View) {
        if let View::Empty = new_configuration {
            if let Some(id) = self.child() {
                self.deactive_child(arena, id);
            }
            self.set_child(None);
            return;
        }

        let configuration = self.to_configuration();

        if let Some(id) = self.child() {
            let child = arena.get_mut(id).unwrap().get_mut().clone();

            if configuration == new_configuration {
                // Skip update child element.
                return;
            } else if configuration.same_type(&new_configuration)
                && configuration.to_keypath() == new_configuration.to_keypath()
            {
                // Same element type and path with different configuration.
                child.update_configuration(new_configuration);
            } else {
                self.deactive_child(arena, id);
                self.inflate_view(arena, new_configuration);
            }
        } else {
            self.inflate_view(arena, new_configuration);
        }
    }

    fn deactive_child(&mut self, arena: &mut Arena<Element>, id: ElementId) {
        id.remove(arena);
    }

    fn inflate_view(&mut self, arena: &mut Arena<Element>, configuration: View) {
        let child_id = configuration.into_element(arena);

        if let Some(child_id) = child_id {
            let element = arena.get(child_id).unwrap().get().clone();

            element.mount(arena, Some(self.to_id()));
        }

        self.set_child(child_id);
    }
}
