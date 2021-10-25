use rltk::{GameState, Rltk};
use specs::prelude::*;

use crate::ecs::components;
use crate::ecs::errors::Result;
use crate::ecs::systems;
use crate::graphics;
use crate::levels::level::{Level, LevelType};
use crate::levels::level_manager::LevelManager;
use crate::maps::Map;

pub struct State {
    pub ecs: World,

    pub level_manager: LevelManager,
    pub current_level: i16,
}

impl State {
    pub fn new() -> State {
        State {
            current_level: 0,
            ecs: World::new(),
            level_manager: LevelManager::new(),
        }
    }

    pub fn register_all_components(&mut self) {
        self.ecs.register::<components::Position>();
        self.ecs.register::<components::Renderable>();
        self.ecs.register::<components::Player>();
        self.ecs.register::<components::Movable>();
        self.ecs.register::<components::AI>();
        self.ecs.register::<components::View>();
        self.ecs.register::<components::ViewMemory>();
    }

    pub fn current_map(&self) -> &Map {
        &self.level_manager.current_level().map
    }

    pub fn current_level(&self) -> &Level {
        self.level_manager.current_level()
    }

    pub fn create_new_level(
        &mut self,
        level_type: LevelType,
        width: usize,
        height: usize,
    ) -> Result<()> {
        self.level_manager
            .crete_new_level(level_type, width, height)?;
        Ok(())
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

        systems::update_view(self, ctx);
        systems::update_view_memory(self, ctx);
        graphics::draw_map_with_fov(self, ctx);
        graphics::draw_entities(self, ctx);
    }
}
