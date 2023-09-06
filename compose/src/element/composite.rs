use super::*;

pub trait CompositeElement: ElementProvider {
    fn build(&mut self) -> View;

    fn child(&self) -> Option<ElementId>;

    fn set_child(&mut self, new: Option<ElementId>);

    fn rebuild(&mut self) {
        let new_configuration = self.build();

        self.update_child(new_configuration);
    }

    fn update_child(&mut self, _new_configuration: View) {
        self.set_child(Some(self.to_id()))
    }
}
