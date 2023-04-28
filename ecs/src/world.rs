use std::collections::BTreeMap;

use uuid::Uuid;

use crate::{componet::Component, system::System};

/// ECS register center.
///
pub struct World {
    /// World register components.
    components: BTreeMap<Uuid, BTreeMap<Uuid, Box<dyn Component>>>,
    /// World register system list
    system_list: Option<BTreeMap<Uuid, Box<dyn System>>>,
}

impl World {
    /// Create a new empty world
    pub fn new() -> Self {
        Self {
            components: Default::default(),
            system_list: Some(Default::default()),
        }
    }
    /// Register new component instance to this world
    pub fn register_component<C: Component + 'static>(&mut self, component: C) -> &mut Self {
        if let Some(system) = self.components.get_mut(component.system()) {
            system.insert(component.id().clone(), Box::new(component));
        } else {
            let mut system = BTreeMap::<Uuid, Box<dyn Component>>::new();

            let system_id = component.system().clone();

            system.insert(component.id().clone(), Box::new(component));

            self.components.insert(system_id, system);
        }

        self
    }
    /// Unregister component from this world
    pub fn unregister_component(&mut self, system_id: &Uuid, component_id: &Uuid) -> &mut Self {
        if let Some(system) = self.components.get_mut(system_id) {
            system.remove(component_id);
        }

        self
    }

    /// Register new system instance to this world
    pub fn register_system<S: System + 'static>(&mut self, system: S) -> &mut Self {
        self.system_list
            .as_mut()
            .unwrap()
            .insert(system.id().clone(), Box::new(system));

        self
    }

    /// Invoke world frame update once.
    pub fn frame_update(&mut self) {
        let mut system_list = self.system_list.take().unwrap();

        for (_, system) in &mut system_list {
            system.update(self)
        }

        self.system_list = Some(system_list);
    }
}
