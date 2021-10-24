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
        graphics::draw_entities(&self, ctx);
    }
}
