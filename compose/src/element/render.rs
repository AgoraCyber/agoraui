use indextree::Arena;

use crate::view::{Configuration, RenderObjectConfiguration, View};

use super::{BuildContext, Element, ElementId, ElementNode, Lifecycle};

pub type RenderObjectElement = ElementNode<dyn RenderObjectConfiguration, ()>;

impl RenderObjectElement {
    pub fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn RenderObjectConfiguration>,
    ) -> ElementId {
        let id = arena.new_node(
            RenderObjectElement {
                id: None,
                config,
                content: (),
            }
            .into(),
        );

        arena
            .get_mut(id)
            .unwrap()
            .get_mut()
            .0
            .borrow_mut()
            .initialize(id);

        id
    }
}

impl Lifecycle for RenderObjectElement {
    fn rebuild(&mut self, _arena: &mut Arena<Element>) {}

    fn to_configuration(&self) -> crate::view::View {
        View::RenderObject(self.config.clone())
    }

    fn update(&mut self, configuration: crate::view::View) {
        if let View::RenderObject(config) = configuration {
            self.config = config
        } else {
            panic!("Update configuration type mismatch, expect RenderObject configuration");
        }
    }
}

impl BuildContext for RenderObjectElement {}
