use rltk::{Console, GameState, Rltk, RGB};
use specs::shred::Fetch;
use specs::{prelude::*, Component};

use crate::ecs::components;
use crate::ecs::systems;
use crate::graphics;
use crate::maps::Map;

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
        self.ecs.register::<components::Position>();
        self.ecs.register::<components::Renderable>();
        self.ecs.register::<components::Player>();
        self.ecs.register::<components::Movable>();
        self.ecs.register::<components::AI>();
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
