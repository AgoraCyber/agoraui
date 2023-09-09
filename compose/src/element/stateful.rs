use indextree::Arena;

use crate::view::{Configuration, State, StatefulConfiguration, View};

use super::{
    component::ComponentElement, BuildContext, Element, ElementId, ElementNode, Lifecycle,
};

#[derive(Debug)]
pub struct StatefulElementContent {
    pub child: Option<ElementId>,
    pub state: Option<Box<dyn State>>,
}

pub type StatefulElement = ElementNode<dyn StatefulConfiguration, StatefulElementContent>;

impl StatefulElement {
    pub fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn StatefulConfiguration>,
    ) -> ElementId {
        let state = config.view.borrow().framework_create_state();

        let id = arena.new_node(
            StatefulElement {
                id: None,
                config,
                content: StatefulElementContent {
                    child: None,
                    state: Some(state),
                },
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

impl Lifecycle for StatefulElement {
    fn rebuild(&mut self, arena: &mut Arena<Element>) {
        self.composite_rebuild(arena);
    }

    fn to_configuration(&self) -> crate::view::View {
        View::Stateful(self.config.clone())
    }

    fn update(&mut self, configuration: crate::view::View) {
        if let View::Stateful(config) = configuration {
            self.config = config
        } else {
            panic!("Update configuration type mismatch, expect Stateful configuration");
        }
    }
}

impl ComponentElement for StatefulElement {
    fn build(&mut self) -> crate::view::View {
        let state = self.content.state.take().unwrap();
        let view = state.framework_build(self);

        self.content.state = Some(state);

        view
    }

    fn set_child(&mut self, new: Option<ElementId>) {
        self.content.child = new;
    }

    fn child(&self) -> Option<ElementId> {
        self.content.child.clone()
    }
}

impl BuildContext for StatefulElement {}
