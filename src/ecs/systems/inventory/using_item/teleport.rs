use specs::{Entity, WriteStorage};

use crate::ecs::components::{self};

pub fn use_teleporting_item(
    _player: Entity,
    user: Entity,
    target_pos: rltk::Point,
    teleportings_effects: &mut WriteStorage<'_, components::TeleportingEffect>,
) {
    teleportings_effects
        .insert(
            user,
            components::TeleportingEffect {
                target_pos: (target_pos.x as usize, target_pos.y as usize),
            },
        )
        .expect("Unable to add teleporting effect");
}
