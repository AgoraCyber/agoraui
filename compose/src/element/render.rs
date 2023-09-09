use indextree::Arena;

use crate::{
    framework::FrameworkContext,
    view::{Configuration, RenderObjectConfiguration, View},
};

use super::{BuildContext, Element, ElementId, ElementNode, Lifecycle};

#[derive(Debug)]
pub struct RenderObjectElementContent {
    pub children: Vec<ElementId>,
}

pub type RenderObjectElement =
    ElementNode<dyn RenderObjectConfiguration, RenderObjectElementContent>;

impl RenderObjectElement {
    pub fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn RenderObjectConfiguration>,
    ) -> ElementId {
        let id = arena.new_node(
            RenderObjectElement {
                id: None,
                config,
                content: RenderObjectElementContent { children: vec![] },
            }
            .into(),
        );

        arena.get_mut(id).unwrap().get_mut().initialize(id);

        id
    }
}

impl Lifecycle for RenderObjectElement {
    fn rebuild(&mut self, build_context: &mut FrameworkContext) {
        let _render_object = self.config.view.borrow().framework_create_render_object();

        let configs = self.config.view.borrow().framework_render_object_children();

        let mut children = vec![];

        for child in configs {
            if let Some(id) = self.inflate_view(build_context, child) {
                children.push(id);
            }
        }

        self.content.children = children;
    }

    fn to_configuration(&self) -> crate::view::View {
        View::RenderObject(self.config.clone())
    }

    fn update(&mut self, _build_context: &mut FrameworkContext, configuration: crate::view::View) {
        if let View::RenderObject(config) = configuration {
            self.config = config
        } else {
            panic!("Update configuration type mismatch, expect RenderObject configuration");
        }
    }
}

impl BuildContext for RenderObjectElement {}
