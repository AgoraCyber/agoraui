use agoraui_compose::*;
use agoraui_compose_macros::Composable;

#[derive(Composable)]
#[allow(dead_code)]
struct TextField {
    pub text: String,
}

impl TextField {
    fn build(&mut self, _context: impl BuildContext) -> impl View {
        Column {
            children: (
                TextField {
                    text: "".to_owned(),
                },
                TextField {
                    text: "".to_owned(),
                },
                if true {
                    TextField {
                        text: "".to_owned(),
                    }
                    .into_any_view()
                } else {
                    Column {
                        children: TextField {
                            text: "".to_owned(),
                        },
                    }
                    .into_any_view()
                },
            ),
        }
    }
}

#[derive(Composable)]
#[allow(dead_code)]
struct Column<Content: View + 'static> {
    pub children: Content,
}

impl<Content: View> Column<Content> {
    fn build(&mut self, _context: impl BuildContext) -> impl View {
        ()
    }
}

struct MockBuildContext(i32);

impl BuildContext for MockBuildContext {
    fn set_state(&mut self) {
        self.0 += 1;
    }
}
