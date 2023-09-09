use std::{cell::RefCell, fmt::Debug, rc::Rc};

use indextree::NodeId;

use crate::{
    framework::FrameworkContext,
    view::{Configuration, View},
};

/// Element id in index tree.
pub type ElementId = NodeId;

pub trait BuildContext {}

pub trait Initializer {
    fn initialize(&mut self, id: ElementId);

    fn to_id(&self) -> Option<ElementId>;
}

/// Framework call this trait to handle element lifecycle.
pub trait Lifecycle: Initializer + Debug {
    fn to_configuration(&self) -> View;

    fn update(&mut self, build_context: &mut FrameworkContext, configuration: View);

    fn rebuild(&mut self, build_context: &mut FrameworkContext);

    /// Mount element into element tree.
    fn mount(&mut self, build_context: &mut FrameworkContext, parent: Option<ElementId>) {
        parent.map(|p| {
            p.append(
                self.to_id().expect("Call initialize first"),
                &mut build_context.element_tree.borrow_mut(),
            )
        });

        self.rebuild(build_context);
    }

    fn update_child(
        &mut self,
        build_context: &mut FrameworkContext,
        child: Option<Element>,
        new_configuration: View,
    ) -> Option<ElementId> {
        if let View::Empty = new_configuration {
            if let Some(child) = child {
                self.deactive_child(build_context, child.to_id().expect("Call initialize first"));
            }

            return None;
        }

        let configuration = self.to_configuration();

        if let Some(mut child) = child {
            if configuration == new_configuration {
                // Skip update child element.
                Some(child.to_id().expect("Call initialize first"))
            } else if configuration.same_type(&new_configuration)
                && configuration.to_keypath() == new_configuration.to_keypath()
            {
                // Same element type and path with different configuration.
                child.update(build_context, new_configuration);
                Some(child.to_id().expect("Call initialize first"))
            } else {
                self.deactive_child(build_context, child.to_id().expect("Call initialize first"));
                self.inflate_view(build_context, new_configuration)
            }
        } else {
            self.inflate_view(build_context, new_configuration)
        }
    }

    fn deactive_child(&mut self, build_context: &mut FrameworkContext, id: ElementId) {
        id.remove(&mut build_context.element_tree.borrow_mut());
    }

    fn inflate_view(
        &mut self,
        build_context: &mut FrameworkContext,
        configuration: View,
    ) -> Option<NodeId> {
        let child_id = configuration.into_element(&mut build_context.element_tree.borrow_mut());

        if let Some(child_id) = child_id {
            let mut element = build_context
                .clone()
                .element_tree
                .borrow_mut()
                .get_mut(child_id)
                .unwrap()
                .get_mut()
                .clone();

            element.mount(
                build_context,
                Some(self.to_id().expect("Call initialize first")),
            );
        }

        child_id
    }
}

/// Element wrapper
#[derive(Debug, Clone)]
pub struct Element(pub Rc<RefCell<dyn Lifecycle>>);

impl<T: Lifecycle + 'static> From<T> for Element {
    fn from(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}

impl Element {
    pub fn mount(&mut self, build_context: &mut FrameworkContext, parent: Option<ElementId>) {
        self.0.borrow_mut().mount(build_context, parent)
    }

    /// Get element mounted id .
    pub fn to_id(&self) -> Option<ElementId> {
        self.0.borrow().to_id()
    }

    fn update(&mut self, build_context: &mut FrameworkContext, configuration: View) {
        self.0.borrow_mut().update(build_context, configuration);
    }

    pub fn initialize(&mut self, id: ElementId) {
        self.0.borrow_mut().initialize(id);
    }
}

#[derive(Debug)]
pub struct ElementNode<T: ?Sized, C> {
    pub id: Option<ElementId>,
    pub config: Configuration<T>,
    pub content: C,
}

impl<T: ?Sized, C> Initializer for ElementNode<T, C> {
    fn initialize(&mut self, id: ElementId) {
        self.id = Some(id);
    }

    fn to_id(&self) -> Option<ElementId> {
        self.id
    }
}
