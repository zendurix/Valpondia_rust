use specs::{Entity, World};

use crate::{
    levels::level::Level,
    maps::rect::Rect,
    rng,
    spawner::{
        items::spawn_healing_potion,
        monsters::{spawn_goblin, spawn_random_monster},
    },
};

use self::{
    items::{
        spawn_fireball_scroll, spawn_magic_missile_scroll, spawn_sleep_scroll,
        spawn_teleport_scroll,
    },
    monsters::{spawn_human, spawn_knight, spawn_orc},
    spawn_tables::SpawnTable,
};

pub mod items;
pub mod monsters;
pub mod player;
pub mod spawn_tables;

pub fn spawn_from_spawn_table(ecs: &mut World, level: &Level, mut spawn_table: SpawnTable) {
    if level.spawn_areas.is_empty() {
        println!("Can't spawn on level without spawn areas!");
        return;
    }

    for spawn_area in level.spawn_areas.iter() {
        let spawn_pack_index = spawn_table.roll_spawn_pack_index(spawn_area.len());
        if let Some(index) = spawn_pack_index {
            let mut spawned_points = vec![];

            for entry in spawn_table.spawn_packs[index].entities.iter() {
                let num = entry.roll_spawn_num();
                let spawn_points = random_spawn_points(num, &spawned_points, spawn_area);

                for (x, y) in spawn_points.into_iter() {
                    if let Some(_ent) =
                        spawn_entity(ecs, &entry.entity_name, x, y, level.level_index)
                    {
                        spawned_points.push((x, y));
                    }
                }
            }
        }
    }
}

fn random_spawn_points(
    num: usize,
    exclude_points: &[(usize, usize)],
    spawn_area: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut spawn_points = vec![];
    for _ in 0..num {
        loop {
            let i = rng::range(0, spawn_area.len() as i32 - 1) as usize;
            let point = spawn_area[i];
            if !spawn_points.contains(&point) && !exclude_points.contains(&point) {
                spawn_points.push(point);
                break;
            }
        }
    }
    spawn_points
}

fn spawn_entity(ecs: &mut World, name: &str, x: usize, y: usize, level: usize) -> Option<Entity> {
    match name {
        "Orc" => Some(spawn_orc(ecs, x, y, level)),
        "Goblin" => Some(spawn_goblin(ecs, x, y, level)),
        "Knight" => Some(spawn_knight(ecs, x, y, level)),
        "Human" => Some(spawn_human(ecs, x, y, level)),

        "Healing potion" => Some(spawn_healing_potion(ecs, x, y, level)),

        "Magic missile scrool" => Some(spawn_magic_missile_scroll(ecs, x, y, level)),
        "Sleep scrool" => Some(spawn_sleep_scroll(ecs, x, y, level)),
        "Fireball scrool" => Some(spawn_fireball_scroll(ecs, x, y, level)),
        "Teleport scrool" => Some(spawn_teleport_scroll(ecs, x, y, level)),
        _ => {
            println!("Cannot spawn {}. Unknown entity", name);
            None
        }
    }
}

fn random_spawn_points_from_room(
    num: usize,
    exclude_points: &[(usize, usize)],
    room: &Rect,
) -> Vec<(usize, usize)> {
    let mut spawn_points = vec![];
    for _ in 0..num {
        loop {
            let x = room.x1 + rng::range(1, room.width() as i32) as usize;
            let y = room.y1 + rng::range(1, room.height() as i32) as usize;
            if !spawn_points.contains(&(x, y)) && !exclude_points.contains(&(x, y)) {
                spawn_points.push((x, y));
                break;
            }
        }
    }
    spawn_points
}

pub fn spawn_random_monsters_and_items_for_room(ecs: &mut World, room: &Rect, level: usize) {
    let rand = rng::roll_dice(1, 10);

    match rand {
        1..=4 => spawn_goblin_room(ecs, room, level),
        5..=7 => spawn_orc_room(ecs, room, level),
        8..=9 => spawn_knight_room(ecs, room, level),
        10 => {
            spawn_random_monster(ecs, room.center().0, room.center().1, level);
        }
        _ => panic!("Wrong random number during monster for room spawning"),
    }
}

fn spawn_goblin_room(ecs: &mut World, room: &Rect, level: usize) {
    let num = rng::range(2, 5) as usize;
    let spawn_points = random_spawn_points_from_room(num, &[], room);
    for (x, y) in spawn_points.iter() {
        spawn_goblin(ecs, *x, *y, level);
    }

    let num = rng::range(1, 2) as usize;
    let item_spawn_points = random_spawn_points_from_room(num, &spawn_points, room);
    spawn_sleep_scroll(ecs, item_spawn_points[0].0, item_spawn_points[0].1, level);
    for (x, y) in item_spawn_points.iter().skip(1) {
        spawn_healing_potion(ecs, *x, *y, level);
    }
}

fn spawn_orc_room(ecs: &mut World, room: &Rect, level: usize) {
    let num = rng::range(1, 2) as usize;
    let spawn_points = random_spawn_points_from_room(num, &[], room);
    for (x, y) in spawn_points.iter() {
        spawn_orc(ecs, *x, *y, level);
    }

    let num = rng::range(1, 2) as usize;
    let item_spawn_points = random_spawn_points_from_room(num, &spawn_points, room);
    spawn_fireball_scroll(ecs, item_spawn_points[0].0, item_spawn_points[0].1, level);
    for (x, y) in item_spawn_points.iter().skip(1) {
        spawn_healing_potion(ecs, *x, *y, level);
    }
}

fn spawn_knight_room(ecs: &mut World, room: &Rect, level: usize) {
    let num = 1;
    let spawn_points = random_spawn_points_from_room(num, &[], room);
    for (x, y) in spawn_points.iter() {
        spawn_knight(ecs, *x, *y, level);
    }

    let num = rng::range(3, 4) as usize;
    let item_spawn_points = random_spawn_points_from_room(num, &spawn_points, room);
    spawn_fireball_scroll(ecs, item_spawn_points[0].0, item_spawn_points[0].1, level);
    spawn_magic_missile_scroll(ecs, item_spawn_points[1].0, item_spawn_points[1].1, level);
    spawn_teleport_scroll(ecs, item_spawn_points[1].0, item_spawn_points[2].1, level);
    for (x, y) in item_spawn_points.iter().skip(3) {
        spawn_healing_potion(ecs, *x, *y, level);
    }
}
