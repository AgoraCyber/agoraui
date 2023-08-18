use std::{
    cell::{Ref, RefCell},
    ptr::NonNull,
    rc::Rc,
};

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
    fn create_state() -> Box<dyn State>;
}

pub trait State {
    fn framework_build(&self) -> View;
}

pub trait CompositeView: ToElement + IntoView {
    fn framework_build(&self) -> View;
}

pub trait RenderObjectView: ToElement + IntoView {}

impl<V: RenderObjectView> From<V> for View {
    fn from(value: V) -> Self {
        View::from_render_object(value)
    }
}

#[repr(C)]
struct ViewVTable {
    /// Create new associated view element.
    to_element: unsafe fn(object: Ref<'_, NonNull<ViewVTable>>, view: View),
}

#[repr(C)]
struct CompositeViewVTable {
    view: ViewVTable,
    build: unsafe fn(object: Ref<'_, NonNull<ViewVTable>>) -> View,
}

#[repr(C)]
struct CompositeWithStateViewVTable {
    view: ViewVTable,
    create_state: unsafe fn(object: Ref<'_, NonNull<ViewVTable>>) -> Box<dyn State>,
}

#[repr(C)]
struct RenderObjectViewVTable {
    view: ViewVTable,
}

impl RenderObjectViewVTable {
    fn new<State: RenderObjectView>() -> Self {
        RenderObjectViewVTable {
            view: ViewVTable {
                to_element: render_object_view_to_element::<State>,
            },
        }
    }
}
impl CompositeViewVTable {
    fn new<State: CompositeView>() -> Self {
        CompositeViewVTable {
            view: ViewVTable {
                to_element: composite_view_to_element::<State>,
            },

            build: composite_view_build::<State>,
        }
    }
}

unsafe fn composite_view_build<State: CompositeView>(object: Ref<'_, NonNull<ViewVTable>>) -> View {
    let v = object.cast::<CompositeRawView<State>>();

    v.as_ref().state.framework_build()
}
unsafe fn composite_view_to_element<State: CompositeView>(
    _object: Ref<'_, NonNull<ViewVTable>>,
    view: View,
) {
    let v = _object.cast::<CompositeRawView<State>>();

    v.as_ref().state.to_element(view);
}

unsafe fn render_object_view_to_element<State: RenderObjectView>(
    _object: Ref<'_, NonNull<ViewVTable>>,
    view: View,
) {
    let v = _object.cast::<RenderObjectRawView<State>>();

    v.as_ref().state.to_element(view);
}

#[repr(C)]
struct RenderObjectRawView<State: RenderObjectView> {
    vtable: RenderObjectViewVTable,
    state: State,
}

#[repr(C)]
struct CompositeRawView<State: CompositeView> {
    vtable: CompositeViewVTable,
    state: State,
}

/// Polymorphic erase view type
#[derive(Debug, Clone)]
pub enum View {
    Empty,
    Composite(AnyCompositeView),
    CompositeWithState(AnyCompositeWithStateView),
    RenderObject(AnyRenderObjectView),
}

impl View {
    /// Create erased type view from [`CompositeView`].
    pub fn from_composite<State: CompositeView>(state: State) -> View {
        let boxed = Box::new(CompositeRawView::<State> {
            vtable: CompositeViewVTable::new::<State>(),
            state,
        });

        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(boxed) as *mut ViewVTable) };

        View::Composite(AnyCompositeView {
            raw_view: Rc::new(RefCell::new(ptr)),
        })
    }
    /// Create erased type view from [`RenderObjectView`].
    pub fn from_render_object<State: RenderObjectView>(state: State) -> View {
        let boxed = Box::new(RenderObjectRawView::<State> {
            vtable: RenderObjectViewVTable::new::<State>(),
            state,
        });

        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(boxed) as *mut ViewVTable) };

        View::RenderObject(AnyRenderObjectView {
            raw_view: Rc::new(RefCell::new(ptr)),
        })
    }

    /// Convert [`AnyView`] to [`Element`]
    pub fn to_element(&self) {
        match self {
            View::Composite(view) => view.to_element(),
            View::CompositeWithState(view) => view.to_element(),
            View::RenderObject(view) => view.to_element(),
            View::Empty => {}
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

#[derive(Debug, Clone)]
pub struct AnyCompositeView {
    raw_view: Rc<RefCell<NonNull<ViewVTable>>>,
}

impl AnyCompositeView {
    pub fn to_element(&self) {
        unsafe {
            let to_element = self.raw_view.borrow().as_ref().to_element;

            to_element(self.raw_view.borrow(), View::Composite(self.clone()));
        }
    }

    /// Build composite view
    pub fn build(&self) -> View {
        unsafe {
            let composite_view = self.raw_view.borrow().cast::<CompositeViewVTable>();

            let build = composite_view.as_ref().build;

            build(self.raw_view.borrow())
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnyCompositeWithStateView {
    raw_view: Rc<RefCell<NonNull<ViewVTable>>>,
}

impl AnyCompositeWithStateView {
    pub fn to_element(&self) {
        unsafe {
            let to_element = self.raw_view.borrow().as_ref().to_element;

            to_element(
                self.raw_view.borrow(),
                View::CompositeWithState(self.clone()),
            );
        }
    }

    /// Build composite view
    pub fn create_state(&self) -> Box<dyn State> {
        unsafe {
            let composite_view = self.raw_view.borrow().cast::<CompositeViewVTable>();

            let build = composite_view.as_ref().build;

            build(self.raw_view.borrow())
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnyRenderObjectView {
    raw_view: Rc<RefCell<NonNull<ViewVTable>>>,
}

impl AnyRenderObjectView {
    pub fn to_element(&self) {
        unsafe {
            let to_element = self.raw_view.borrow().as_ref().to_element;

            to_element(self.raw_view.borrow(), View::RenderObject(self.clone()));
        }
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
