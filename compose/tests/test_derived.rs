use agoraui_compose::{framework::FrameworkContext, prelude::*};
use indextree::Arena;

#[derive(Debug, PartialEq, Stateless)]
struct Text {}

impl Text {
    fn build(&self, _context: &impl BuildContext) -> impl IntoView {
        Label {}
    }
}

#[derive(Debug, PartialEq, Render)]
struct Label {}

impl Label {
    fn create_render_object(&self) -> impl RenderObjectLifecycle {
        LabelRenderObject {}
    }

    fn render_object_children(&self) -> Vec<View> {
        vec![]
    }
}

#[derive(Debug)]
struct LabelRenderObject {}

impl RenderObjectLifecycle for LabelRenderObject {}

#[test]
fn test_into_view() {
    let mut arena = Arena::new();
    let view = Text {}.into_view();

    let element_id = view.into_element(&mut arena);

    assert!(element_id.is_some());
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
    let mut context = FrameworkContext::default();
    let view = Text {}.into_view();

    let element_id = view
        .into_element(&mut context.element_tree.borrow_mut())
        .unwrap();

    let element = context
        .element_tree
        .clone()
        .borrow_mut()
        .get_mut(element_id)
        .unwrap()
        .get()
        .clone();

    element.mount(&mut context, None);

    assert!(context
        .element_tree
        .borrow()
        .get(element_id)
        .unwrap()
        .get()
        .to_id()
        .is_some());

    
}
