use agoraui_compose::*;

#[derive(Debug, Stateless, PartialEq)]
struct Text {}

impl Text {
    fn build(&self, _: &mut impl BuildContext) -> impl IntoView {}
}

#[derive(Debug, Stateful, PartialEq)]
struct InputField {}

impl InputField {
    fn create_state(&self) -> InputFieldState {
        InputFieldState {}
    }
}

#[derive(Debug, State, PartialEq)]
struct InputFieldState {}

impl InputFieldState {
    fn build(&self, _: &mut impl BuildContext) -> impl IntoView {}
}

#[test]
fn test_key_path() {
    let view = InputField {}.into_view();

    assert_eq!(view.to_key_path(), view.to_key_path());

    let view2 = InputField {}.into_view();

    assert_ne!(view.to_key_path(), view2.to_key_path());
}

#[test]
fn test_to_element() {
    let view = InputField {}.into_view();

    let mut element = view.to_element();

    assert_eq!(element.view(), view);

    element.mount(None);
}
