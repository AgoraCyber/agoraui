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

struct LabelRenderObject {}

impl RenderObject for LabelRenderObject {}

#[test]
fn test_into_view() {
    let mut arena = Arena::new();
    let view = Text {}.into_view();

    let element_id = view.into_element(&mut arena);

    assert!(element_id.is_some());
}

#[test]
fn test_render_object() {
    let mut arena = Arena::new();
    let view = Label {}.into_view();

    let element_id = view.into_element(&mut arena);

    assert!(element_id.is_some());

    let element = arena.get(element_id.unwrap()).unwrap().get();

    match element {
        Element::Render(_) => {}
        _ => assert!(false, "expect Render element"),
    }
}

#[test]
fn test_arena_ancestors() {
    _ = pretty_env_logger::try_init();

    let mut arena = Arena::new();

    let a = arena.new_node(1);
    let b = arena.new_node(2);
    let c = arena.new_node(3);

    a.append(b, &mut arena);

    b.append(c, &mut arena);

    for i in c.ancestors(&arena) {
        log::debug!("{}", i);
    }

    c.ancestors(&arena);

    assert_eq!(arena.get(c).unwrap().parent().unwrap(), b);
}

#[test]
fn test_mount() {
    let mut arena = Arena::new();
    let view = Label {}.into_view();

    let element_id = view.into_element(&mut arena).unwrap();

    let element = arena.get(element_id).unwrap().get().clone();

    element.mount(&mut arena, None);

    assert!(arena.get(element_id).unwrap().get().mounted());
}
