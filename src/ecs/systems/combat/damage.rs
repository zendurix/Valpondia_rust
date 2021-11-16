use crate::{components, ecs::game_state::RunState, gamelog::GameLog};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, components::Hp>,
        WriteStorage<'a, components::SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    // Using a scope to make the borrow checker happy
    {
        let hps = ecs.read_storage::<components::Hp>();
        let entities = ecs.entities();
        let players = ecs.read_storage::<components::Player>();

        for (entity, hp) in (&entities, &hps).join() {
            if hp.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => dead.push(entity),
                    Some(_) => {
                        let mut runstate = ecs.write_resource::<RunState>();
                        *runstate = RunState::GameOver;
                    }
                }
            }
        }
    }

    {
        let names = ecs.read_storage::<components::Name>();
        let mut gamelog = ecs.write_resource::<GameLog>();

        for victim in dead.iter() {
            if let Some(name) = names.get(*victim) {
                gamelog
                    .entries
                    .push(format!("{} dies in agony :( ", name.name));
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
