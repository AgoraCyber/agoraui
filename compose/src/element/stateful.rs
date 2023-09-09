use std::cell::RefCell;

use indextree::Arena;

use crate::{
    framework::FrameworkContext,
    view::{Configuration, RenderObjectId, State, StatefulConfiguration, View},
};

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
                id: RefCell::new(None),
                config: RefCell::new(config),
                content: RefCell::new(StatefulElementContent {
                    child: None,
                    state: Some(state),
                }),
            }
            .into(),
        );

        arena.get_mut(id).unwrap().get_mut().initialize(id);

        id
    }
}

impl Lifecycle for StatefulElement {
    fn first_render_object_id(&self, build_context: &FrameworkContext) -> Option<RenderObjectId> {
        self.composite_first_render_object_id(build_context)
    }
    fn to_render_object_id(&self) -> Option<RenderObjectId> {
        None
    }
    fn rebuild(&self, build_context: &mut FrameworkContext) {
        self.composite_rebuild(build_context);
    }

    fn to_configuration(&self) -> crate::view::View {
        View::Stateful(self.config.borrow().clone())
    }

    fn update(&self, _build_context: &mut FrameworkContext, configuration: crate::view::View) {
        if let View::Stateful(config) = configuration {
            *self.config.borrow_mut() = config
        } else {
            panic!("Update configuration type mismatch, expect Stateful configuration");
        }
    }
}

impl ComponentElement for StatefulElement {
    fn build(&self) -> crate::view::View {
        let state = self.content.borrow_mut().state.take().unwrap();
        let view = state.framework_build(self);

        self.content.borrow_mut().state = Some(state);

        view
    }

    fn set_child(&self, new: Option<ElementId>) {
        self.content.borrow_mut().child = new;
    }

    fn child(&self) -> Option<ElementId> {
        self.content.borrow().child.clone()
    }
}

impl BuildContext for StatefulElement {}
