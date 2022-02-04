use crate::{
    components,
    ecs::{game_state::RunState, State},
    gamelog::GameLog,
};
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

pub fn delete_the_dead(gs: &mut State) {
    let mut dead: Vec<Entity> = Vec::new();
    // Using a scope to make the borrow checker happy
    {
        let hps = gs.ecs.read_storage::<components::Hp>();
        let entities = gs.ecs.entities();
        let players = gs.ecs.read_storage::<components::Player>();
        let final_boss = gs.ecs.read_storage::<components::FinalBoss>();

        for (entity, hp) in (&entities, &hps).join() {
            if hp.hp < 1 {
                let player = players.get(entity);
                match player {
                    None => dead.push(entity),
                    Some(_) => {
                        let mut runstate = gs.ecs.write_resource::<RunState>();
                        *runstate = RunState::GameOver;
                    }
                }

                if final_boss.get(entity).is_some() {
                    gs.game_won = true;
                }
            }
        }
    }

    {
        let names = gs.ecs.read_storage::<components::Name>();
        let mut gamelog = gs.ecs.write_resource::<GameLog>();

        for victim in dead.iter() {
            if let Some(name) = names.get(*victim) {
                gamelog.entries.push(format!("{} dies.", name.name));
            }
        }
    }

    for victim in dead {
        gs.ecs.delete_entity(victim).expect("Unable to delete");
    }
}
