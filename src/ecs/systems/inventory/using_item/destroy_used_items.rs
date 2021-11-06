use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::ecs::components::{self};

pub struct DestroyUsedItems {}

impl<'a> System<'a> for DestroyUsedItems {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, components::WantsToUseItem>,
        ReadStorage<'a, components::Usable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            entities,
            mut wants_to_use,
            usables,
        ) = data;

        for wants_use_item in wants_to_use.join() {
            let item = wants_use_item.item;
            if let Some(usable) = usables.get(item) {
                if usable.destoyed_on_use {
                    entities.delete(item).expect("Item Delete failed");
                }
            }
        }
        wants_to_use.clear();
    }
}
