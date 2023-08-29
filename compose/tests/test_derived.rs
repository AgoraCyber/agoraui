use agoraui_compose::*;

#[derive(Composite, PartialEq, Debug)]
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

#[derive(CompositeWithState, PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
struct MockRenderObject {}

impl IRenderObjectView for MockRenderObject {}

impl ToElement for MockRenderObject {
    fn to_element(&self, _view: View) -> Element {
        todo!()
    }
}

impl IntoView for MockRenderObject {
    #[track_caller]
    fn into_view(self) -> View {
        let caller = std::panic::Location::caller();
        View::from_render_object(format!("{}", caller), self)
    }
}

impl ToAny for MockRenderObject {
    fn to_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl AnyEq for MockRenderObject {
    fn eq(&self, other: &dyn std::any::Any) -> bool {
        self == other.downcast_ref::<MockRenderObject>().unwrap()
    }
}

impl ToKey for MockRenderObject {
    fn to_key(&self) -> &str {
        ""
    }
}

#[test]
fn test_element_mount() {
    let col = Column { children: vec![] }.into_view();

    let mut element = col.to_element().unwrap();

    element.mount(None)
}

#[test]
fn test_view_partial_eq() {
    let lhs = Text {
        label: "Hello".to_string(),
    }
    .into_view();

    let rhs = Text {
        label: "World".to_string(),
    }
    .into_view();

    assert_eq!(lhs, lhs.clone());

    assert_ne!(lhs, rhs);
}

#[derive(Composite, PartialEq, Debug)]
#[allow(unused)]
struct KeyTester {
    i: i32,
}

impl KeyTester {
    fn build(&self) -> impl IntoView {
        if self.i > 10 {
            Text {
                label: "Hello".to_string(),
            }
            .into_view()
        } else {
            Text {
                label: "World".to_string(),
            }
            .into_view()
        }
    }
}

#[test]
fn test_view_key() {
    _ = pretty_env_logger::try_init();

    let mut tester = KeyTester { i: 0 };

    let lhs = tester.build().into_view();

    tester.i = 11;

    let rhs = tester.build().into_view();

    assert_ne!(rhs.to_key(), lhs.to_key());
}
