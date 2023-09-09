use indextree::Arena;

use crate::{
    framework::FrameworkContext,
    view::{Configuration, StatelessConfiguration, View},
};

use super::{
    component::ComponentElement, BuildContext, Element, ElementId, ElementNode, Lifecycle,
};

pub type StatelessElement = ElementNode<dyn StatelessConfiguration, Option<ElementId>>;

impl StatelessElement {
    pub fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn StatelessConfiguration>,
    ) -> ElementId {
        let id = arena.new_node(
            StatelessElement {
                id: None,
                config,
                content: None,
            }
            .into(),
        );

        arena.get_mut(id).unwrap().get_mut().initialize(id);

        id
    }
}

impl Lifecycle for StatelessElement {
    fn rebuild(&mut self, build_context: &mut FrameworkContext) {
        self.composite_rebuild(build_context);
    }

    fn to_configuration(&self) -> crate::view::View {
        View::Stateless(self.config.clone())
    }

    fn update(&mut self, _build_context: &mut FrameworkContext, configuration: crate::view::View) {
        if let View::Stateless(config) = configuration {
            self.config = config
        } else {
            panic!("Update configuration type mismatch, expect Stateless configuration");
        }
    }
}

impl ComponentElement for StatelessElement {
    fn build(&mut self) -> crate::view::View {
        self.config.view.clone().borrow().framework_build(self)
    }

    fn set_child(&mut self, new: Option<ElementId>) {
        self.content = new;
    }

    fn child(&self) -> Option<ElementId> {
        self.content.clone()
    }
}

impl BuildContext for StatelessElement {}
