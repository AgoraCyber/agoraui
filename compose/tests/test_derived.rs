use agoraui_compose::*;

#[allow(unused)]
#[derive(Stateless)]
struct Label {
    pub text: String,
}

impl Label {
    fn build(&self, _context: &mut dyn BuildContext) -> impl View {}
}

#[derive(Stateful)]
struct TextField {}

impl TextField {
    fn create_view_state(&self) -> impl ViewState {
        TextFieldState {}
    }
}

#[derive(State)]
struct TextFieldState {}

impl TextFieldState {
    fn build(&mut self, _context: &mut dyn BuildContext) -> impl View {}
}
