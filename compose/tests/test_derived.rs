use agoraui_compose::prelude::*;
use indextree::Arena;

#[derive(Debug, PartialEq, Stateless)]
struct Text {}

impl Text {
    fn build(&self, _context: &mut impl BuildContext) -> impl IntoView {
        Label {}
    }
}

#[derive(Debug, PartialEq, Leaf)]
struct Label {}

impl Label {
    fn create_render_object(&self) -> impl RenderObject {
        LabelRenderObject {}
    }
}

#[derive(Debug)]
struct LabelRenderObject {}

impl RenderObject for LabelRenderObject {}

#[test]
fn test_into_view() {
    let mut arena = Arena::new();
    let view = Text {}.into_view();

    let element_id = view.into_element(&mut arena);

    assert!(element_id.is_some());
}
