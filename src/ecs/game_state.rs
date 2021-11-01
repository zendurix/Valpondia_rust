use rltk::{GameState, Rltk};
use specs::prelude::*;

use crate::ecs::components;
use crate::ecs::errors::Result;
use crate::ecs::systems;
use crate::graphics;
use crate::levels::level::{Level, LevelType};
use crate::levels::level_manager::LevelManager;
use crate::maps::Map;

use super::systems::ai::AISystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    Running,
    Paused,
}

pub struct State {
    pub ecs: World,
    pub run_state: RunState,

    pub player_pos: components::Position,

    pub level_manager: LevelManager,
    pub current_level: i16,
}

impl State {
    pub fn new() -> State {
        State {
            run_state: RunState::Paused,
            current_level: 0,
            ecs: World::new(),
            level_manager: LevelManager::new(),
            player_pos: components::Position {
                x: 0,
                y: 0,
                level: 0,
            },
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
        self.ecs.register::<components::Name>();
        self.ecs.register::<components::OccupiesTile>();
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
        self.run_state = systems::player::try_player_turn(self, ctx);
        if self.run_state == RunState::Running {
            // systems::ai::ai_random_mov::move_all(self, ctx);
            systems::ai_main(self, ctx);
            systems::move_all(self, ctx);
            systems::update_view(self, true);
            systems::update_view_memory(self, ctx);
        }

        ctx.cls();
        graphics::draw_map_with_fov(self, ctx);
        // graphics::draw_map_without_fov(self.current_map(), ctx);
        graphics::draw_entities(self, ctx);
    }
}
