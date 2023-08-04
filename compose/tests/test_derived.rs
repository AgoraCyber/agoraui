use agoraui_compose::*;
use agoraui_compose_macros::Composable;

#[derive(Composable)]
#[allow(dead_code)]
struct TextField {
    #[state]
    pub text: String,
}

impl TextField {
    fn build(&mut self, _context: &impl BuildContext) -> impl View {
        TextField { text: "".into() }
    }
}

#[test]
fn test_derived() {}
