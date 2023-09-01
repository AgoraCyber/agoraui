use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::StatelessElement;

use super::*;

/// Stateless ui element configration.
pub trait StatelessConfigration: ToElement + IntoView + ToAny + AnyEq + Debug {
    /// Rebuild this view configration.
    fn framework_build(&self, element: &mut StatelessElement) -> View;
}

pub type Stateless = Configration<dyn StatelessConfigration>;

pub fn stateless_to_view<T: StatelessConfigration + 'static, K: Into<KeyPath>>(
    keypath: K,
    config: T,
) -> View {
    View::Stateless(Configration {
        keypath: keypath.into(),
        configration: Rc::new(RefCell::new(config)),
    })
}
