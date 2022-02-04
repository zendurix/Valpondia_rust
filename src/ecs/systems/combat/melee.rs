use crate::{components, ecs::game_state::GameLog};
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
        ReadStorage<'a, components::Equipped>,
        ReadStorage<'a, components::MeleeDamageBonus>,
        ReadStorage<'a, components::DefenseBonus>,
        WriteExpect<'a, GameLog>,
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
            equippeds,
            melee_bonuses,
            def_bonuses,
            mut gamelog,
        ) = data;

        for (entity, wants_melee, name, hp, stats) in
            (&entities, &wants_melee, &names, &hps, &combat_stats).join()
        {
            if hp.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                let target_hp = hps.get(wants_melee.target).unwrap();
                if target_hp.hp > 0 {
                    let target_name = names.get(wants_melee.target).unwrap();

                    let mut damage = stats.attack;

                    for (_equipped, melee_bonus) in (&equippeds, &melee_bonuses)
                        .join()
                        .filter(|(equipped_by, _bonus)| equipped_by.owner == entity)
                    {
                        damage += melee_bonus.power;
                    }

                    let mut target_defense = target_stats.defense;

                    for (_equipped, def_bonus) in (&equippeds, &def_bonuses)
                        .join()
                        .filter(|(equipped_by, _bonus)| equipped_by.owner == wants_melee.target)
                    {
                        target_defense += def_bonus.defense;
                    }

                    damage -= target_defense;

                    if damage <= 0 {
                        gamelog.entries.push(format!(
                            "{} Attacks doesnt affect  {} (0 dmg)",
                            &name.name, &target_name.name
                        ));
                    } else {
                        gamelog.entries.push(format!(
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
