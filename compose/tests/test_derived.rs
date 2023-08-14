use agoraui_compose::*;

#[allow(unused)]
#[derive(Stateless, Clone)]
struct Label {
    pub text: String,
}

impl Label {
    fn build(&self, _context: &mut dyn BuildContext) -> impl View {}
}

#[derive(Stateful, Clone)]
struct TextField {}

impl TextField {
    fn create_view_state(&self) -> impl ViewState {
        TextFieldState {
            _widget: self.clone(),
        }
    }
}

#[derive(State)]
struct TextFieldState {
    _widget: TextField,
}

impl TextFieldState {
    fn build(&mut self, _context: &mut dyn BuildContext) -> impl View {}
}

impl ViewStateLifecycle for TextFieldState {}

#[derive(Stateless)]
struct AppView {}

impl AppView {
    fn build(&self, _context: &mut dyn BuildContext) -> impl View {
        (
            Label {
                text: "Hello".to_string(),
            },
            TextField {},
        )
    }
}

#[test]
fn test_create_view() {
    let view = AppView {};

    let element = view.create_element();
}
