use agoraui_compose::prelude::*;
use indextree::Arena;

#[derive(Debug, Stateless)]
struct Text {}

impl Text {
    fn build(&self, _context: &mut impl BuildContext) -> impl IntoView {
        Label {}
    }
}

#[derive(Debug, Leaf)]
struct Label {}

impl Label {
    fn create_render_object(&self) -> impl RenderObject {
        LabelRenderObject {}
    }
}

struct LabelRenderObject {}

impl RenderObject for LabelRenderObject {}

#[test]
fn test_into_view() {
    let mut arena = Arena::new();
    let view = Text {}.into_view();

    let element_id = view.to_element(&mut arena);

    assert!(element_id.is_some());
}

#[test]
fn test_render_object() {
    let mut arena = Arena::new();
    let view = Label {}.into_view();

    let element_id = view.to_element(&mut arena);

    assert!(element_id.is_some());

    let element = arena.get(element_id.unwrap()).unwrap().get();

    match element {
        Element::Render(_) => {}
        _ => assert!(false, "expect Render element"),
    }
}
