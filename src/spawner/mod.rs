use lazy_static::__Deref;
use specs::World;

use crate::{
    maps::{rect::Rect, Map},
    rng,
    spawner::monsters::{spawn_goblin, spawn_random_monster},
};

use self::monsters::{spawn_knight, spawn_orc};

pub mod items;
pub mod monsters;
pub mod player;

pub fn spawn_random_monsters_for_room(ecs: &mut World, room: &Rect, level: usize) {
    let map = ecs.fetch_mut::<Map>().clone();
    let rand = rng::roll_dice(1, 10);

    match rand {
        1..=4 => spawn_goblin_room(ecs, room, &map, level),
        5..=7 => spawn_orc_room(ecs, room, &map, level),
        8..=9 => spawn_knight_room(ecs, room, &map, level),
        10 => {
            spawn_random_monster(ecs, room.center().0, room.center().1, level);
        }
        _ => panic!("Wrong random number during monster for room spawning"),
    }
}

fn random_spawn_points(num: usize, room: &Rect, map: &Map) -> Vec<(usize, usize)> {
    let mut spawn_points = vec![];
    for _ in 0..num {
        loop {
            let x = room.x1 + rng::range(1, room.width() as i32) as usize;
            let y = room.y1 + rng::range(1, room.height() as i32) as usize;
            let index = map.xy_to_index(x, y);
            if !spawn_points.contains(&(x, y)) {
                spawn_points.push((x, y));
                break;
            }
        }
    }
    spawn_points
}

fn spawn_goblin_room(ecs: &mut World, room: &Rect, map: &Map, level: usize) {
    let num = rng::range(2, 5) as usize;
    let mut spawn_points = random_spawn_points(num, room, map);
    for (x, y) in spawn_points {
        spawn_goblin(ecs, x, y, level);
    }
}

fn spawn_orc_room(ecs: &mut World, room: &Rect, map: &Map, level: usize) {
    let num = rng::range(1, 2) as usize;
    let mut spawn_points = random_spawn_points(num, room, map);
    for (x, y) in spawn_points {
        spawn_orc(ecs, x, y, level);
    }
}

fn spawn_knight_room(ecs: &mut World, room: &Rect, map: &Map, level: usize) {
    let num = 1;
    let mut spawn_points = random_spawn_points(num, room, map);
    for (x, y) in spawn_points {
        spawn_knight(ecs, x, y, level);
    }
}
