use agoraui_compose::*;

#[derive(Composite)]
#[allow(unused)]
struct Column {
    children: Vec<AnyView>,
}

impl Column {
    fn build(&self) -> impl ToAnyView {
        let i = 0;
        Column {
            children: view_list![
                if i > 10 {
                    Text {
                        label: "Hello0".to_string(),
                    }
                } else {
                    Text {
                        label: "Hello10".to_string(),
                    }
                },
                Text {
                    label: "Hello".to_string(),
                },
            ],
        }
    }
}

#[derive(Composite)]
#[allow(unused)]
struct Text {
    pub label: String,
}

impl Text {
    fn build(&self) -> impl ToAnyView {
        MockRenderObject {}
    }
}

struct MockRenderObject {}

impl RenderObjectView for MockRenderObject {}

impl ToElement for MockRenderObject {
    fn to_element(&self, _view: AnyView) -> () {
        todo!()
    }
}

impl ToAnyView for MockRenderObject {
    fn to_any_view(self) -> AnyView {
        AnyView::from_render_object(self)
    }
}

#[test]
fn test_column() {
    let col = Column { children: vec![] }.to_any_view();

    col.to_element();
}
