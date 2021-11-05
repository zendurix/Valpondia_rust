use crate::components;
use rltk::console;
use specs::prelude::*;

pub struct MeleeCombatSystem {}

#[allow(clippy::type_complexity)]
impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, components::WantsToMeleeAtack>,
        ReadStorage<'a, components::Name>,
        ReadStorage<'a, components::Hp>,
        ReadStorage<'a, components::CombatBaseStats>,
        WriteStorage<'a, components::SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            entities,
            mut wants_melee,
            names,
            hps,
            combat_stats,
            mut inflict_damage,
        ) = data;

        for (_entity, wants_melee, name, hp, stats) in
            (&entities, &wants_melee, &names, &hps, &combat_stats).join()
        {
            if hp.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                let target_hp = hps.get(wants_melee.target).unwrap();
                if target_hp.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();

                    let damage = i32::max(0, stats.attack - target_stats.defense);

                    if damage == 0 {
                        console::log(&format!(
                            "{} Attacks doesnt affect  {} (0 dmg)",
                            &name.name, &target_name.name
                        ));
                    } else {
                        console::log(&format!(
                            "{} hits {}, for {} hp.",
                            &name.name, &target_name.name, damage
                        ));
                        components::SufferDamage::new_damage(
                            &mut inflict_damage,
                            wants_melee.target,
                            damage,
                        );
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
