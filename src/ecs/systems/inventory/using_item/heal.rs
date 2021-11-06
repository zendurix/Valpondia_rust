use specs::{Entity, WriteStorage};

use crate::ecs::components::{self};

pub fn use_heal_item<'a>(
    _player: Entity,
    _user: Entity,
    heal: &components::Heal,
    targets: Vec<Entity>,
    heals_effects: &mut WriteStorage<'a, components::HealEffect>,
) {
    for target in targets {
        heals_effects
            .insert(
                target,
                components::HealEffect {
                    heal_power: heal.heal_power,
                },
            )
            .expect("Unable to add heal effect");
    }
}
