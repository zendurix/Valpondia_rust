use specs::{Entity, WriteStorage};

use crate::ecs::components::{self};

pub fn use_damage_item<'a>(
    _player: Entity,
    _user: Entity,
    dmg: &components::InflictsDamage,
    targets: Vec<Entity>,
    suffers_damages: &mut WriteStorage<'a, components::SufferDamage>,
) {
    for target in targets {
        components::SufferDamage::new_damage(suffers_damages, target, dmg.damage);
    }
}
