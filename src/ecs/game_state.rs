use rltk::{Console, GameState, Rltk, RGB};
use specs::{prelude::*, Component};

use crate::ecs::components::*;

pub struct State {
    pub ecs: World,
    pub current_level: i16,
}

impl State {
    pub fn new() -> State {
        let mut ecs = World::new();

        State {
            current_level: 0,
            ecs,
        }
    }

    pub fn register_all_components(&mut self) {
        self.ecs.register::<Position>();
        self.ecs.register::<Renderable>();
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
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.render_tiles(ctx);
    }
}
