use std::cell::RefCell;

use indextree::Arena;

use crate::{
    framework::FrameworkContext,
    view::{Configuration, RenderObject, RenderObjectConfiguration, RenderObjectId, View},
};

use super::{BuildContext, Element, ElementId, ElementNode, Initializer, Lifecycle};

#[derive(Debug)]
pub struct RenderObjectElementContent {
    pub render_object_id: Option<RenderObjectId>,
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
                id: RefCell::new(None),
                config: RefCell::new(config),
                content: RefCell::new(RenderObjectElementContent {
                    render_object_id: None,
                    children: vec![],
                }),
            }
            .into(),
        );

        arena.get_mut(id).unwrap().get_mut().initialize(id);

        id
    }

    fn attach_render_object(
        &self,
        build_context: &mut FrameworkContext,
        mut render_object: RenderObject,
    ) {
        let render_object_id = build_context
            .render_tree
            .borrow_mut()
            .new_node(render_object.clone());

        render_object.initialize(render_object_id);

        self.content.borrow_mut().render_object_id = Some(render_object_id);

        let ancestor_render_object_id =
            self.find_ancestor_render_object_element(&mut build_context.element_tree.borrow_mut());

        if let Some(ancestor_render_object_id) = ancestor_render_object_id {
            ancestor_render_object_id.append(
                render_object_id,
                &mut build_context.render_tree.borrow_mut(),
            );
        }
    }

    fn find_ancestor_render_object_element(
        &self,
        arena: &mut Arena<Element>,
    ) -> Option<RenderObjectId> {
        let element_id = self.to_id().expect("Call mount method first");

        let mut ancestors = element_id.ancestors(arena);

        // Skip self
        ancestors.next();

        for id in ancestors {
            let render_object_id = arena.get(id).unwrap().get().to_render_object_id();

            if render_object_id.is_some() {
                return render_object_id;
            }
        }

        None
    }
}

impl Lifecycle for RenderObjectElement {
    fn first_render_object_id(&self, _build_context: &FrameworkContext) -> Option<RenderObjectId> {
        self.content.borrow().render_object_id
    }
    fn to_render_object_id(&self) -> Option<RenderObjectId> {
        self.content.borrow().render_object_id.clone()
    }
    fn rebuild(&self, build_context: &mut FrameworkContext) {
        let render_object = self
            .config
            .borrow()
            .view
            .borrow()
            .framework_create_render_object();

        self.attach_render_object(build_context, render_object);

        let configs = self
            .config
            .borrow()
            .view
            .borrow()
            .framework_render_object_children();

        let mut children = vec![];

        for child in configs {
            if let Some(id) = self.inflate_view(build_context, child) {
                children.push(id);
            }
        }

        self.content.borrow_mut().children = children;
    }

    fn to_configuration(&self) -> crate::view::View {
        View::RenderObject(self.config.borrow().clone())
    }

    fn update(&self, _build_context: &mut FrameworkContext, configuration: crate::view::View) {
        if let View::RenderObject(config) = configuration {
            *self.config.borrow_mut() = config
        } else {
            panic!("Update configuration type mismatch, expect RenderObject configuration");
        }
    }
}

impl BuildContext for RenderObjectElement {}
