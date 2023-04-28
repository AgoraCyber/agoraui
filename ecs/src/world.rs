use std::collections::BTreeMap;

use uuid::Uuid;

use crate::{
    component::Component,
    system::{System, SystemId},
};

/// ECS register center.
///
pub struct World {
    /// World register components.
    components: BTreeMap<Uuid, BTreeMap<Uuid, Box<dyn Component>>>,
    /// World register system list
    system_list: Option<Vec<Box<dyn System>>>,
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
    pub fn unregister_component(&mut self, id: &Uuid) -> &mut Self {
        for (_, system) in self.components.iter_mut() {
            if system.remove(id).is_some() {
                break;
            }
        }

        self
    }

    /// Get system const component list in this world
    pub fn get_system_components<C: Component + 'static>(&self, system_id: &Uuid) -> Vec<&C> {
        if let Some(system) = self.components.get(system_id) {
            system
                .iter()
                .map(|(_, c)| c.as_any().downcast_ref().unwrap())
                .collect::<Vec<_>>()
        } else {
            Default::default()
        }
    }

    /// Get system mut component list in this world
    pub fn get_system_components_mut<C: Component + 'static>(
        &mut self,
        system_id: &Uuid,
    ) -> Vec<&mut C> {
        if let Some(system) = self.components.get_mut(system_id) {
            system
                .iter_mut()
                .map(|(_, c)| c.as_any_mut().downcast_mut().unwrap())
                .collect::<Vec<_>>()
        } else {
            Default::default()
        }
    }

    /// Register new system instance to this world
    pub fn register_system<S: System + SystemId + 'static>(&mut self, system: S) -> &mut Self {
        self.system_list.as_mut().unwrap().push(Box::new(system));

        self
    }

    /// Invoke world frame update once.
    pub fn frame_update(&mut self) {
        let mut system_list = self.system_list.take().unwrap();

        for system in &mut system_list {
            system.update(self)
        }

        self.system_list = Some(system_list);
    }
}
