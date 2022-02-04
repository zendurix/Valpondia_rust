use specs::{Entities, Entity, Join, ReadExpect, System, WriteExpect, WriteStorage};

use crate::{
    ecs::{components::{self}, game_state::GameLog},
};

/// used for heal potions.

pub struct HealSystem {}

impl<'a> System<'a> for HealSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, components::HealEffect>,
        WriteStorage<'a, components::Hp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            entities,
            player,
            mut gamelog,
            mut heals,
            mut hps,
        ) = data;

        for (entity, heal, hp) in (&entities, &heals, &mut hps).join() {
            hp.hp = (hp.hp + heal.heal_power).min(hp.max_hp);

            if entity == *player {
                gamelog
                    .entries
                    .push(format!("You are healed for {} hp.", heal.heal_power));
            }
        }
        heals.clear();
    }
}
