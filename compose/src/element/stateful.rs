use super::*;

pub type StatefulElement = ElementWrapper<dyn StatefulConfiguration, Option<ElementId>>;

impl StatefulElement {
    pub(crate) fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn StatefulConfiguration>,
    ) -> ElementId {
        let id = arena.new_node(Element::Stateful(Rc::new(RefCell::new(StatefulElement {
            id: None,
            config,
            mounted: false,
            content: None,
        }))));

        match arena.get_mut(id).unwrap().get_mut() {
            Element::Stateful(e) => e.borrow_mut().id = Some(id),
            _ => {}
        }

        id
    }
}

impl CompositeElement for StatefulElement {
    fn build(&mut self) -> View {
        todo!()
    }

    fn child(&self) -> Option<ElementId> {
        self.content.clone()
    }

    fn set_child(&mut self, new: Option<ElementId>) {
        self.content = new
    }
}
