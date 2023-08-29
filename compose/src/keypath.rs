use std::panic::Location;

///  identity of render element in the view tree
#[derive(Debug, Clone, PartialEq)]
pub enum KeyPath {
    Empty,
    /// use rust caller location as unique identities
    TrackCaller(&'static Location<'static>),
}

impl From<&'static Location<'static>> for KeyPath {
    fn from(value: &'static Location<'static>) -> Self {
        KeyPath::TrackCaller(value)
    }
}
