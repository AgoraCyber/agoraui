/// UI render element.
pub trait Elemement<'a> {}

pub struct StatelessElement {}

impl<'a> Elemement<'a> for StatelessElement {}
