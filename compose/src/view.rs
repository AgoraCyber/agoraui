use std::{
    cell::{Ref, RefCell},
    ptr::NonNull,
    rc::Rc,
};

pub trait ToElement {
    fn to_element(&self, view: AnyView) -> ();
}

pub trait ToAnyView {
    fn to_any_view(self) -> AnyView;
}

pub trait CompositeView: ToElement + ToAnyView {
    fn framework_build(&self) -> AnyView;
}

pub trait RenderObjectView: ToElement + ToAnyView {}

impl<V: RenderObjectView> From<V> for AnyView {
    fn from(value: V) -> Self {
        AnyView::from_render_object(value)
    }
}

#[repr(C)]
struct ViewVTable {
    /// Create new associated view element.
    to_element: unsafe fn(object: Ref<'_, NonNull<ViewVTable>>, view: AnyView),
}

#[repr(C)]
struct CompositeViewVTable {
    view: ViewVTable,
    build: unsafe fn(object: Ref<'_, NonNull<ViewVTable>>) -> AnyView,
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

unsafe fn composite_view_build<State: CompositeView>(
    object: Ref<'_, NonNull<ViewVTable>>,
) -> AnyView {
    let v = object.cast::<CompositeRawView<State>>();

    v.as_ref().state.framework_build()
}
unsafe fn composite_view_to_element<State: CompositeView>(
    _object: Ref<'_, NonNull<ViewVTable>>,
    view: AnyView,
) {
    let v = _object.cast::<CompositeRawView<State>>();

    v.as_ref().state.to_element(view);
}

unsafe fn render_object_view_to_element<State: RenderObjectView>(
    _object: Ref<'_, NonNull<ViewVTable>>,
    view: AnyView,
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
pub enum AnyView {
    Composite(AnyCompositeView),
    RenderObject(AnyRenderObjectView),
}

impl AnyView {
    /// Create erased type view from [`CompositeView`].
    pub fn from_composite<State: CompositeView>(state: State) -> AnyView {
        let boxed = Box::new(CompositeRawView::<State> {
            vtable: CompositeViewVTable::new::<State>(),
            state,
        });

        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(boxed) as *mut ViewVTable) };

        AnyView::Composite(AnyCompositeView {
            raw_view: Rc::new(RefCell::new(ptr)),
        })
    }
    /// Create erased type view from [`RenderObjectView`].
    pub fn from_render_object<State: RenderObjectView>(state: State) -> AnyView {
        let boxed = Box::new(RenderObjectRawView::<State> {
            vtable: RenderObjectViewVTable::new::<State>(),
            state,
        });

        let ptr = unsafe { NonNull::new_unchecked(Box::into_raw(boxed) as *mut ViewVTable) };

        AnyView::RenderObject(AnyRenderObjectView {
            raw_view: Rc::new(RefCell::new(ptr)),
        })
    }

    /// Convert [`AnyView`] to [`Element`]
    pub fn to_element(&self) {
        match self {
            AnyView::Composite(view) => view.to_element(),
            AnyView::RenderObject(view) => view.to_element(),
        }
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

            to_element(self.raw_view.borrow(), AnyView::Composite(self.clone()));
        }
    }

    /// Build composite view
    pub fn build(&self) -> AnyView {
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

            to_element(self.raw_view.borrow(), AnyView::RenderObject(self.clone()));
        }
    }
}

#[macro_export]
macro_rules! view_list {
    () => (
       Vec::new()
    );
    ($($x:expr),+ $(,)?) => (
        vec![$($x.to_any_view()),+]
    );
}

#[cfg(test)]
mod tsts {

    use std::{cell::RefCell, rc::Rc};

    use crate::{AnyView, CompositeView, ToAnyView, ToElement};

    #[derive(Default)]
    struct Mock {
        count: Rc<RefCell<i32>>,
    }

    impl CompositeView for Mock {
        fn framework_build(&self) -> AnyView {
            *self.count.borrow_mut() += 1;

            Mock {
                ..Default::default()
            }
            .to_any_view()
        }
    }

    impl ToElement for Mock {
        fn to_element(&self, _view: crate::AnyView) -> () {
            *self.count.borrow_mut() += 1;
        }
    }

    impl ToAnyView for Mock {
        fn to_any_view(self) -> crate::AnyView {
            AnyView::from_composite(self)
        }
    }

    #[test]
    fn test_to_element() {
        let count = Rc::new(RefCell::new(0));
        let mock = Mock {
            count: count.clone(),
        }
        .to_any_view();

        mock.to_element();

        assert_eq!(*count.borrow(), 1);

        // Reference the same view
        mock.clone().to_element();

        assert_eq!(*count.borrow(), 2);

        if let AnyView::Composite(view) = &mock {
            view.build();
        }

        assert_eq!(*count.borrow(), 3);
    }
}
