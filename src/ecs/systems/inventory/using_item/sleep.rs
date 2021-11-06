use specs::{Entity, WriteStorage};

use crate::ecs::components::{self};

pub fn use_sleep_item<'a>(
    _player: Entity,
    _user: Entity,
    sleep: &components::Sleeping,
    targets: Vec<Entity>,
    sleepings_effects: &mut WriteStorage<'a, components::SleepingEffect>,
) {
    for target in targets {
        sleepings_effects
            .insert(
                target,
                components::SleepingEffect {
                    duration: sleep.duration,
                },
            )
            .expect("Unable to add sleep effect");
    }
}
