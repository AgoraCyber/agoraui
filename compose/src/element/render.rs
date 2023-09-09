use crate::view::RenderObject;

use super::*;

#[derive(Default, Debug)]
pub struct RenderElementContent {
    pub render_object: Option<Box<dyn RenderObject>>,
}

pub type RenderElement = ElementWrapper<dyn RenderObjectConfiguration, RenderElementContent>;

impl RenderElement {
    pub(crate) fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn RenderObjectConfiguration>,
    ) -> ElementId {
        let id = arena.new_node(Element::Render(Rc::new(RefCell::new(RenderElement {
            id: None,
            config,
            mounted: false,
            content: RenderElementContent::default(),
        }))));

        match arena.get_mut(id).unwrap().get_mut() {
            Element::Render(e) => e.borrow_mut().id = Some(id),
            _ => {}
        }

        id
    }
}

impl ToConfiguration for RenderElement {
    fn to_configuration(&self) -> View {
        View::RenderObject(self.config.clone())
    }

    fn update_configuration(&mut self, view: View) {
        if let View::RenderObject(config) = view {
            self.config = config
        }
    }
}

impl GetChild for RenderElement {
    fn child(&self) -> Option<ElementId> {
        None
    }
}

impl Lifecycle for RenderElement {
    fn rebuild(&mut self, _arena: &mut Arena<Element>) {
        let render_object = self.config.view.borrow().framework_create_render_object();
        self.attach_render_object(render_object);
    }
}

impl RenderElement {
    fn attach_render_object(&mut self, render_object: Box<dyn RenderObject>) {
        self.content.render_object = Some(render_object);
    }
}
