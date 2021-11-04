use lazy_static::__Deref;
use rltk::field_of_view_set;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

use crate::{ecs::components, maps::Map};

pub struct ViewSystem {}

impl<'a> System<'a> for ViewSystem {
    type SystemData = (
        ReadStorage<'a, components::Position>,
        WriteStorage<'a, components::View>,
        ReadExpect<'a, Map>,
    );

    fn run(&mut self, data: Self::SystemData) {
        #[rustfmt::skip]
        let (
            positions ,
            mut views,
            current_map
        ) = data;

        for (pos, view) in (&positions, &mut views)
            .join()
            .filter(|(_pos, view)| view.should_update)
        {
            view.visible_tiles.clear();
            view.visible_tiles = field_of_view_set(
                rltk::Point::new(pos.x, pos.y),
                view.range as i32,
                current_map.deref(),
            );
            view.visible_tiles.retain(|p| {
                p.x >= 0
                    && p.x < current_map.width as i32
                    && p.y >= 0
                    && p.y < current_map.height as i32
            });
            view.should_update = false;
        }
    }
}

pub struct ViewMemorySystem {}
impl<'a> System<'a> for ViewMemorySystem {
    type SystemData = (
        ReadStorage<'a, components::View>,
        WriteStorage<'a, components::ViewMemory>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (views, mut views_memories) = data;
        for (view, view_memory) in (&views, &mut views_memories).join() {
            view_memory.seen_tiles.extend(view.visible_tiles.clone());
        }
    }
}

/*
* My old implementation from C++.

fn calculate_field_of_view(start: rltk::Point, range: usize, map: &Map) -> HashSet<rltk::Point> {
    let mut visible_tiles = HashSet::<rltk::Point>::new();

    let accuracy = 0.00625;
    let degre_step = 0.125;

    let max = (360.0 / degre_step) as usize;
    visible_tiles.insert(start);

    for i in 0..max {
        let alpha = i as f32 * degre_step;
        visible_tiles.extend(tracer(map, start, alpha, range, accuracy));
    }

    visible_tiles
}

fn tracer(
    map: &Map,
    start: rltk::Point,
    mut angle: f32,
    view_range: usize,
    accuracy: f32,
) -> HashSet<rltk::Point> {
    let mut tracer = start;
    let mut visible_points = HashSet::<rltk::Point>::new();

    let quarter = angle_quarter(&mut angle);

    let angle_rad = angle.to_radians();
    let b_add = angle_rad.tan() * accuracy;

    let mut a = 0.0;
    let mut b = 0.0;

    let mut step_counter = 0;

    if angle == 0.0 || angle == 180.0 {
        while step_counter < view_range {
            if let Some(tile) = map.try_get_tile_at_xy(tracer.x as usize, tracer.y as usize) {
                visible_points.insert(tracer);
                if tile.blocks_visibility() {
                    break;
                }
                step_tracer(&mut tracer, quarter, TracerStepDir::Vertical);
                step_counter += 1;
            } else {
                break;
            }
        }
    } else {
        while step_counter < view_range {
            if let Some(tile) = map.try_get_tile_at_xy(tracer.x as usize, tracer.y as usize) {
                visible_points.insert(tracer);
                if tile.blocks_visibility() {
                    break;
                }
                a += accuracy;
                b += b_add;

                if a >= 1.0 && b >= 1.0 {
                    b -= 1.0;
                    a = 0.0;
                    step_tracer(&mut tracer, quarter, TracerStepDir::Diagonal);
                    step_counter += 2;
                } else if b >= 1.0 {
                    b -= 1.0;
                    step_tracer(&mut tracer, quarter, TracerStepDir::Vertical);
                    step_counter += 1;
                } else if a >= 1.0 {
                    a = 0.0;
                    step_tracer(&mut tracer, quarter, TracerStepDir::Horizontal);
                    step_counter += 1;
                }
            } else {
                break;
            }
        }
    }

    visible_points
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Quarter {
    I,
    II,
    III,
    IV,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TracerStepDir {
    Vertical,
    Horizontal,
    Diagonal,
}

fn angle_quarter(angle: &mut f32) -> Quarter {
    if *angle >= 0.0 && *angle < 90.0 {
        Quarter::I
    } else if *angle >= 90.0 && *angle < 180.0 {
        *angle -= 90.0;
        Quarter::II
    } else if *angle >= 180.0 && *angle < 270.0 {
        *angle -= 180.0;
        Quarter::III
    } else if *angle >= 270.0 && *angle < 360.0 {
        *angle -= 270.0;
        Quarter::IV
    } else {
        // todo ERROR
        Quarter::I
    }
}

fn step_tracer(tracer: &mut rltk::Point, quarter: Quarter, dir: TracerStepDir) {
    match quarter {
        Quarter::I => match dir {
            TracerStepDir::Vertical => {
                tracer.y -= 1;
            }
            TracerStepDir::Horizontal => {
                tracer.x += 1;
            }
            TracerStepDir::Diagonal => {
                tracer.y -= 1;
                tracer.x += 1;
            }
        },
        Quarter::II => match dir {
            TracerStepDir::Vertical => {
                tracer.y += 1;
            }
            TracerStepDir::Horizontal => {
                tracer.x += 1;
            }
            TracerStepDir::Diagonal => {
                tracer.y += 1;
                tracer.x += 1;
            }
        },
        Quarter::III => match dir {
            TracerStepDir::Vertical => {
                tracer.y += 1;
            }
            TracerStepDir::Horizontal => {
                tracer.x -= 1;
            }
            TracerStepDir::Diagonal => {
                tracer.y += 1;
                tracer.x -= 1;
            }
        },
        Quarter::IV => match dir {
            TracerStepDir::Vertical => {
                tracer.y -= 1;
            }
            TracerStepDir::Horizontal => {
                tracer.x -= 1;
            }
            TracerStepDir::Diagonal => {
                tracer.y -= 1;
                tracer.x -= 1;
            }
        },
    }
}

*/
