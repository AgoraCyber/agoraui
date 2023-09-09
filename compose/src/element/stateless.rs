use std::cell::RefCell;

use indextree::Arena;

use crate::{
    framework::FrameworkContext,
    view::{Configuration, RenderObjectId, StatelessConfiguration, View},
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
                id: RefCell::new(None),
                config: RefCell::new(config),
                content: RefCell::new(None),
            }
            .into(),
        );

        arena.get_mut(id).unwrap().get_mut().initialize(id);

        id
    }
}

impl Lifecycle for StatelessElement {
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
        View::Stateless(self.config.borrow().clone())
    }

    fn update(&self, _build_context: &mut FrameworkContext, configuration: crate::view::View) {
        if let View::Stateless(config) = configuration {
            *self.config.borrow_mut() = config
        } else {
            panic!("Update configuration type mismatch, expect Stateless configuration");
        }
    }
}

impl ComponentElement for StatelessElement {
    fn build(&self) -> crate::view::View {
        self.config
            .borrow()
            .view
            .clone()
            .borrow()
            .framework_build(self)
    }

    fn set_child(&self, new: Option<ElementId>) {
        *self.content.borrow_mut() = new;
    }

    fn child(&self) -> Option<ElementId> {
        self.content.borrow().clone()
    }
}

impl BuildContext for StatelessElement {}
