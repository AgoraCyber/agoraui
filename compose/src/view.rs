use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

use crate::element::Element;

/// Widget must implement this trait to convert self to [`Element`]
pub trait ToElement {
    fn to_element(&self, view: View) -> Element;
}

pub trait IntoView {
    fn into_view(self) -> View;
}

pub trait ToAny {
    fn to_any(&self) -> &dyn Any;
}

pub trait AnyEq {
    fn eq(&self, other: &dyn Any) -> bool;
}

pub trait ToKey {
    fn to_key(&self) -> &str;
}

/// View with local state must implement this trait
pub trait ICompositeWithStateView: ToElement + IntoView + ToAny + AnyEq + Debug {
    fn framework_create_state(&self) -> Box<dyn State>;
}

pub trait State {
    fn framework_build(&self) -> View;
}

pub trait ICompositeView: ToElement + IntoView + ToAny + AnyEq + Debug {
    fn framework_build(&self) -> View;
}

pub trait IRenderObjectView: ToElement + IntoView + ToAny + AnyEq + Debug {}

/// Polymorphic erase view type
#[derive(Clone, PartialEq, Debug)]
pub enum View {
    Empty,
    Composite(CompositeView),
    CompositeWithState(CompositeWithStateView),
    RenderObject(RenderObjectView),
}

impl View {
    /// Create erased type view from [`CompositeView`].
    pub fn from_composite<State: ICompositeView + 'static>(id: String, state: State) -> View {
        View::Composite(CompositeView {
            id,
            raw_view: Rc::new(RefCell::new(Box::new(state))),
        })
    }

    pub fn from_composite_with_state<State: ICompositeWithStateView + 'static>(
        id: String,
        state: State,
    ) -> View {
        View::CompositeWithState(CompositeWithStateView {
            id,
            raw_view: Rc::new(RefCell::new(Box::new(state))),
        })
    }
    /// Create erased type view from [`RenderObjectView`].
    pub fn from_render_object<State: IRenderObjectView + 'static>(
        id: String,
        state: State,
    ) -> View {
        View::RenderObject(RenderObjectView {
            id,
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

    pub fn to_key(&self) -> Option<&str> {
        match self {
            View::Composite(view) => Some(&view.id),
            View::CompositeWithState(view) => Some(&view.id),
            View::RenderObject(view) => Some(&view.id),
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

#[derive(Clone, Debug)]
pub struct CompositeView {
    id: String,
    raw_view: Rc<RefCell<Box<dyn ICompositeView + 'static>>>,
}

impl CompositeView {
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

impl PartialEq for CompositeView {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.raw_view.borrow();
        let rhs = other.raw_view.borrow();

        lhs.to_any().type_id() == rhs.to_any().type_id() && self.raw_view.borrow().eq(rhs.to_any())
    }
}

#[derive(Clone, Debug)]
pub struct CompositeWithStateView {
    id: String,
    raw_view: Rc<RefCell<Box<dyn ICompositeWithStateView + 'static>>>,
}

impl CompositeWithStateView {
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

impl PartialEq for CompositeWithStateView {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.raw_view.borrow();
        let rhs = other.raw_view.borrow();

        lhs.to_any().type_id() == rhs.to_any().type_id() && self.raw_view.borrow().eq(rhs.to_any())
    }
}

#[derive(Clone, Debug)]
pub struct RenderObjectView {
    id: String,
    raw_view: Rc<RefCell<Box<dyn IRenderObjectView + 'static>>>,
}

impl RenderObjectView {
    pub fn to_element(&self) -> Element {
        self.raw_view
            .borrow()
            .to_element(View::RenderObject(self.clone()))
    }
}

impl PartialEq for RenderObjectView {
    fn eq(&self, other: &Self) -> bool {
        let lhs = self.raw_view.borrow();
        let rhs = other.raw_view.borrow();

        lhs.to_any().type_id() == rhs.to_any().type_id() && self.raw_view.borrow().eq(rhs.to_any())
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

    #[derive(Default, PartialEq, Debug)]
    struct Mock {
        count: Rc<RefCell<i32>>,
    }

    impl ICompositeView for Mock {
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
        #[track_caller]
        fn into_view(self) -> crate::View {
            let caller = std::panic::Location::caller();
            View::from_composite(format!("{}", caller), self)
        }
    }

    impl ToAny for Mock {
        fn to_any(&self) -> &dyn Any {
            self
        }
    }

    impl AnyEq for Mock {
        fn eq(&self, _other: &dyn Any) -> bool {
            self == _other.downcast_ref::<Mock>().unwrap()
        }
    }

    impl ToKey for Mock {
        fn to_key(&self) -> &str {
            ""
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
