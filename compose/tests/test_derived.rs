use agoraui_compose::*;

#[derive(Composite)]
#[allow(unused)]
struct Column {
    children: Vec<View>,
}

impl Column {
    fn build(&self) -> impl IntoView {
        let i = 0;
        Column {
            children: view_list![
                if i > 10 {
                    Text {
                        label: "Hello0".to_string(),
                    }
                    .into_view()
                } else {
                    ().into_view()
                },
                Text {
                    label: "Hello".to_string(),
                },
            ],
        }
    }
}

#[derive(CompositeWithState)]
#[allow(unused)]
struct Text {
    pub label: String,
}

impl Text {
    fn create_state(&self) -> TextState {
        TextState {}
    }
}

#[derive(State)]
struct TextState {}

impl TextState {
    fn build(&self) -> impl IntoView {
        MockRenderObject {}
    }
}

struct MockRenderObject {}

impl IRenderObjectView for MockRenderObject {}

impl ToElement for MockRenderObject {
    fn to_element(&self, _view: View) -> Element {
        todo!()
    }
}

impl IntoView for MockRenderObject {
    fn into_view(self) -> View {
        View::from_render_object(self)
    }
}

#[test]
fn test_element_mount() {
    let col = Column { children: vec![] }.into_view();

    let mut element = col.to_element().unwrap();

    element.mount(None)
}
