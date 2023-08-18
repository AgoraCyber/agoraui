use std::{cell::RefCell, rc::Rc};

use crate::element::Element;

/// Widget must implement this trait to convert self to [`Element`]
pub trait ToElement {
    fn to_element(&self, view: View) -> Element;
}

pub trait IntoView {
    fn into_view(self) -> View;
}

/// View with local state must implement this trait
pub trait CompositeWithStateView: ToElement + IntoView {
    fn framework_create_state(&self) -> Box<dyn State>;
}

pub trait State {
    fn framework_build(&self) -> View;
}

pub trait CompositeView: ToElement + IntoView {
    fn framework_build(&self) -> View;
}

pub trait RenderObjectView: ToElement + IntoView {}

/// Polymorphic erase view type
#[derive(Clone)]
pub enum View {
    Empty,
    Composite(AnyCompositeView),
    CompositeWithState(AnyCompositeWithStateView),
    RenderObject(AnyRenderObjectView),
}

impl View {
    /// Create erased type view from [`CompositeView`].
    pub fn from_composite<State: CompositeView + 'static>(state: State) -> View {
        View::Composite(AnyCompositeView {
            raw_view: Rc::new(RefCell::new(Box::new(state))),
        })
    }

    pub fn from_composite_with_state<State: CompositeWithStateView + 'static>(
        state: State,
    ) -> View {
        View::CompositeWithState(AnyCompositeWithStateView {
            raw_view: Rc::new(RefCell::new(Box::new(state))),
        })
    }
    /// Create erased type view from [`RenderObjectView`].
    pub fn from_render_object<State: RenderObjectView + 'static>(state: State) -> View {
        View::RenderObject(AnyRenderObjectView {
            raw_view: Rc::new(RefCell::new(Box::new(state))),
        })
    }

    /// Convert [`AnyView`] to [`Element`]
    pub fn to_element(&self) -> Option<Element> {
        match self {
            View::Composite(view) => Some(view.to_element()),
            View::CompositeWithState(view) => Some(view.to_element()),
            View::RenderObject(view) => Some(view.to_element()),
            View::Empty => None,
        }
    }
}

impl IntoView for View {
    fn into_view(self) -> View {
        // [`AnyView`] just return self.
        self
    }
}

impl IntoView for () {
    fn into_view(self) -> View {
        return View::Empty;
    }
}

#[derive(Clone)]
pub struct AnyCompositeView {
    raw_view: Rc<RefCell<Box<dyn CompositeView + 'static>>>,
}

impl AnyCompositeView {
    pub fn to_element(&self) -> Element {
        self.raw_view
            .borrow()
            .to_element(View::Composite(self.clone()))
    }

    /// Build composite view
    pub fn build(&self) -> View {
        self.raw_view.borrow().framework_build()
    }
}

#[derive(Clone)]
pub struct AnyCompositeWithStateView {
    raw_view: Rc<RefCell<Box<dyn CompositeWithStateView + 'static>>>,
}

impl AnyCompositeWithStateView {
    pub fn to_element(&self) -> Element {
        self.raw_view
            .borrow()
            .to_element(View::CompositeWithState(self.clone()))
    }

    /// Build composite view
    pub fn create_state(&self) -> Box<dyn State> {
        self.raw_view.borrow().framework_create_state()
    }
}

#[derive(Clone)]
pub struct AnyRenderObjectView {
    raw_view: Rc<RefCell<Box<dyn RenderObjectView + 'static>>>,
}

impl AnyRenderObjectView {
    pub fn to_element(&self) -> Element {
        self.raw_view
            .borrow()
            .to_element(View::RenderObject(self.clone()))
    }
}

#[macro_export]
macro_rules! view_list {
    () => (
       Vec::new()
    );
    ($($x:expr),+ $(,)?) => (
        vec![$($x.into_view()),+]
    );
}

#[cfg(test)]
mod tsts {

    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[derive(Default)]
    struct Mock {
        count: Rc<RefCell<i32>>,
    }

    impl CompositeView for Mock {
        fn framework_build(&self) -> View {
            *self.count.borrow_mut() += 1;

            Mock {
                ..Default::default()
            }
            .into_view()
        }
    }

    impl ToElement for Mock {
        fn to_element(&self, _view: View) -> Element {
            *self.count.borrow_mut() += 1;

            Element::Empty
        }
    }

    impl IntoView for Mock {
        fn into_view(self) -> crate::View {
            View::from_composite(self)
        }
    }

    #[test]
    fn test_to_element() {
        let count = Rc::new(RefCell::new(0));
        let mock = Mock {
            count: count.clone(),
        }
        .into_view();

        mock.to_element();

        assert_eq!(*count.borrow(), 1);

        // Reference the same view
        mock.clone().to_element();

        assert_eq!(*count.borrow(), 2);

        if let View::Composite(view) = &mock {
            view.build();
        }

        assert_eq!(*count.borrow(), 3);
    }
}
