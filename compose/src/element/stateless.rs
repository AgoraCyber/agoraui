use super::*;

pub type StatelessElement = ElementWrapper<dyn StatelessConfiguration, Option<ElementId>>;

impl StatelessElement {
    pub(crate) fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn StatelessConfiguration>,
    ) -> ElementId {
        let id = arena.new_node(Element::Stateless(StatelessElement {
            id: None,
            config,
            content: None,
        }));

        match arena.get_mut(id).unwrap().get_mut() {
            Element::Stateless(e) => e.id = Some(id),
            _ => {}
        }

        id
    }
}

impl CompositeElement for StatelessElement {
    fn build(&mut self) -> View {
        self.config.view.clone().borrow().framework_build(self)
    }

    fn child(&self) -> Option<ElementId> {
        self.content.clone()
    }

    fn set_child(&mut self, new: Option<ElementId>) {
        self.content = new
    }
}
