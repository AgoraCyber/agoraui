use super::*;

pub type RenderElement = ElementWrapper<dyn RenderObjectConfiguration>;

impl RenderElement {
    pub(crate) fn new(
        arena: &mut Arena<Element>,
        config: Configuration<dyn RenderObjectConfiguration>,
    ) -> ElementId {
        let id = arena.new_node(Element::Render(RenderElement {
            id: None,
            config,
            content: (),
        }));

        match arena.get_mut(id).unwrap().get_mut() {
            Element::Render(e) => e.id = Some(id),
            _ => {}
        }

        id
    }
}
