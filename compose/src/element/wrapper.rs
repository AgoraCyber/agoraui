use super::*;

#[derive(Debug)]
pub struct ElementWrapper<T: ?Sized, C = ()> {
    pub id: Option<ElementId>,
    pub config: Configuration<T>,
    pub content: C,
}

impl<T: ?Sized, C> BuildContext for ElementWrapper<T, C> {}

impl<T: ?Sized, C> ElementService for ElementWrapper<T, C> {
    fn to_id(&self) -> ElementId {
        self.id.clone().unwrap()
    }
}
