use specs::{Entities, Entity, Join, ReadExpect, System, WriteExpect, WriteStorage};

use crate::{
    ecs::{components::{self}, game_state::GameLog},
};

/// used for heal potions.

pub struct TeleportSystem {}

impl<'a> System<'a> for TeleportSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, components::TeleportingEffect>,
        WriteStorage<'a, components::Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            entities,
            player,
            mut gamelog,
            mut teleports,
            mut positions,
        ) = data;

        for (entity, teleport, pos) in (&entities, &teleports, &mut positions).join() {
            pos.x = teleport.target_pos.0;
            pos.y = teleport.target_pos.1;

            if entity == *player {
                gamelog.entries.push("You are teleported.".to_string());
            }
        }
        teleports.clear();
    }
}
