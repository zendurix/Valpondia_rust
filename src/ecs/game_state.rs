use rltk::{Console, GameState, Rltk, RGB};
use specs::shred::Fetch;
use specs::{prelude::*, Component};

use crate::data::map::Map;
use crate::ecs::components::*;
use crate::ecs::systems;
use crate::graphics;

pub struct State {
    pub ecs: World,
    pub current_level: i16,
}

impl State {
    pub fn new() -> State {
        let ecs = World::new();

        State {
            current_level: 0,
            ecs,
        }
    }

    pub fn register_all_components(&mut self) {
        self.ecs.register::<Position>();
        self.ecs.register::<Renderable>();
        self.ecs.register::<Player>();
        self.ecs.register::<Movable>();
        self.ecs.register::<AI>();
    }

    pub fn render_tiles(&self, ctx: &mut Rltk) {
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (position, render) in (&positions, &renderables).join() {
            if position.level == self.current_level {
                ctx.set(position.x, position.y, render.fg, render.bg, render.ascii);
            }
        }
    }

    /// equivalent to -> &Map
    pub fn map(&self) -> Fetch<Map> {
        self.ecs.fetch::<Map>()
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        systems::player::try_player_turn(self, ctx);
        if ctx.key.is_some() {
            systems::ai_random_mov::move_all(self, ctx);
            systems::move_all(self, ctx);
        }
        ctx.cls();

        let map = self.ecs.fetch::<Map>();
        graphics::draw_map(&map, ctx);
        self.render_tiles(ctx);
    }
}
