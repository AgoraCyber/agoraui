use libecs::{
    derive::*,
    system::{System, SystemId},
    Uuid,
};
use serde::{Deserialize, Serialize};

#[component(PositionSystem)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Position {
    x: u64,
    y: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[system_id("Position")]
struct PositionSystem;

impl System for PositionSystem {
    fn update(&mut self, _world: &mut libecs::world::World) {}
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use libecs::{component::Component, world::World, Uuid};

    use crate::*;

    #[test]
    fn test_derive() {
        _ = pretty_env_logger::try_init();

        let pos = Position::new(1, 2);

        log::debug!("{}", serde_json::to_string(&pos).unwrap());

        let pos = Position::new_with_id(Uuid::new_v4(), 1, 2);

        log::debug!("{}", serde_json::to_string(&pos).unwrap());

        assert_eq!(
            Uuid::parse_str("4028b9ae-b6b1-9af1-454b-16ce28c1ee69").unwrap(),
            *pos.system()
        );
    }

    #[test]
    fn test_register() {
        _ = pretty_env_logger::try_init();

        let p1 = Position::new_with_id(
            Uuid::from_str("b41c2ebb-5c9f-4210-8ad5-aca5f695a4e5").unwrap(),
            1,
            2,
        );

        let p2 = Position::new_with_id(
            Uuid::from_str("9f4d8212-8ea0-418d-b6de-b7c5c8965681").unwrap(),
            3,
            4,
        );

        for _ in 0..10 {
            let mut world = World::new();

            world
                .register_component(p1.clone())
                .register_component(p2.clone())
                .register_system(PositionSystem::default());

            world.frame_update();

            let pos: Vec<&Position> = world.get_system_components(PositionSystem::id());

            assert_eq!(*pos[0], p2);

            assert_eq!(*pos[1], p1);
        }
    }
}
