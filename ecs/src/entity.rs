use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The entity is the components container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// [`id`](Uuid) for this entity
    pub id: Uuid,
    /// List of bound component IDs.
    pub components: BTreeSet<Uuid>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            components: Default::default(),
        }
    }
}

impl Entity {
    /// Create new entity with providing [`id`](Uuid).
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            components: Default::default(),
        }
    }

    /// Add new component to entity
    pub fn add_component(&mut self, id: Uuid) -> &mut Self {
        self.components.insert(id);
        self
    }

    /// Remove component from this entity
    pub fn remove_component(&mut self, id: &Uuid) -> &mut Self {
        self.components.remove(id);
        self
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::Entity;

    #[test]
    fn test_serde() {
        _ = pretty_env_logger::try_init();

        let mut entity = Entity::default();
        entity
            .add_component(Uuid::new_v4())
            .add_component(Uuid::new_v4());

        log::debug!("{}", serde_json::to_string_pretty(&entity).unwrap());
    }
}
