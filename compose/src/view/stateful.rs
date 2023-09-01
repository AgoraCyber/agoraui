use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::StatefulElement;

use super::*;

/// Stateless ui element configration.
pub trait StatefulConfigration: ToElement + IntoView + ToAny + AnyEq + Debug {
    /// Create new state object
    fn framework_create_state(&self) -> Box<dyn State>;
}

pub trait State {
    /// Rebuild ui configuration.
    fn framework_build(&self, element: &mut StatefulElement) -> View;
}

pub type Stateful = Configration<dyn StatefulConfigration>;

/// Convert [`Stateful`] configuration to [`View`] configuration.
pub fn stateful_to_view<T: StatefulConfigration + 'static, K: Into<KeyPath>>(
    keypath: K,
    config: T,
) -> View {
    View::Stateful(Configration {
        keypath: keypath.into(),
        configration: Rc::new(RefCell::new(config)),
    })
}
