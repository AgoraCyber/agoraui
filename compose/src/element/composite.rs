use super::*;

pub trait CompositeElement: ElementProvider + GetChild + UpdateChild {
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
}
