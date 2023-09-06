use super::*;

#[derive(Debug)]
pub struct ElementWrapper<T: ?Sized, C = ()> {
    pub id: Option<ElementId>,
    pub config: Configuration<T>,
    pub mounted: bool,
    pub content: C,
}

impl<T: ?Sized, C> BuildContext for ElementWrapper<T, C> {}

impl<T: ?Sized, C> ElementProvider for ElementWrapper<T, C> {
    fn to_id(&self) -> ElementId {
        self.id.clone().unwrap()
    }

    fn mounted(&self) -> bool {
        self.mounted
    }

    fn set_mount_flag(&mut self, flag: bool) {
        self.mounted = flag;
    }
}
