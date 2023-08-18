use std::{
    cell::{Ref, RefCell},
    ptr::NonNull,
    rc::Rc,
};

///
pub trait IBuildContext {}

#[repr(C)]
struct BuildContextVTable {
    set_state: unsafe fn(object: Ref<'_, NonNull<BuildContextVTable>>),
}

pub enum Element {
    Empty,
}

#[derive(Debug, Clone)]
pub struct BuildContext {
    raw: Rc<RefCell<NonNull<BuildContextVTable>>>,
}

impl BuildContext {
    pub fn set_state(&self) {
        unsafe {
            let set_state = self.raw.borrow().as_ref().set_state;

            set_state(self.raw.borrow())
        }
    }
}
