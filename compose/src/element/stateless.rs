use super::*;

pub type StatelessElement = ElementWrapper<dyn StatelessConfiguration, Option<ElementId>>;

impl StatelessElement {
    pub(crate) fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn StatelessConfiguration>,
    ) -> ElementId {
        let id = arena.new_node(Element::Stateless(Rc::new(RefCell::new(
            StatelessElement {
                id: None,
                config,
                mounted: false,
                content: None,
            },
        ))));

        match arena.get_mut(id).unwrap().get_mut() {
            Element::Stateless(e) => e.borrow_mut().id = Some(id),
            _ => {}
        }

        id
    }
}

impl ToConfiguration for StatelessElement {
    fn to_configuration(&self) -> View {
        View::Stateless(self.config.clone())
    }

    fn update_configuration(&mut self, view: View) {
        if let View::Stateless(config) = view {
            self.config = config
        }
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
