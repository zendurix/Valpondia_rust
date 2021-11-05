use lazy_static::__Deref;
use rltk::{console, GameState, Rltk};
use specs::prelude::*;

use crate::ecs::components;
use crate::ecs::errors::Result;
use crate::ecs::systems;
use crate::graphics;
use crate::levels::level::{Level, LevelType};
use crate::levels::level_manager::LevelManager;
use crate::maps::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    Running,
    Paused,
}

/// State global resources (stored in rltk)
/// rltk::Point - player position
/// Entity - player entity (id)
/// Map - current map

pub struct State {
    pub ecs: World,
    pub run_state: RunState,

    pub player_pos: components::Position,

    pub level_manager: LevelManager,
    pub current_level: usize,
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
        self.ecs.register::<components::BlocksTile>();
        self.ecs.register::<components::Hp>();
        self.ecs.register::<components::CombatBaseStats>();
        self.ecs.register::<components::WantsToMeleeAtack>();
        self.ecs.register::<components::SufferDamage>();
    }

    pub fn current_map(&self) -> &Map {
        &self.level_manager.current_level().map
    }
    pub fn current_map_mut(&mut self) -> &mut Map {
        &mut self.level_manager.current_level_mut().map
    }

    pub fn current_level(&self) -> &Level {
        self.level_manager.current_level()
    }
    pub fn current_level_mut(&mut self) -> &mut Level {
        self.level_manager.current_level_mut()
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

    pub fn set_level_as_curent(&mut self, level_index: usize) {
        if self.level_manager.levels.len() > level_index {
            if let Some(current_map) = self.ecs.try_fetch::<Map>() {
                let map = current_map.deref().clone();
                self.level_manager.levels[self.current_level].map = map;
            }

            self.current_level = level_index;
            self.ecs
                .insert(self.level_manager.levels[level_index].map.clone());
        }
    }

    fn run_player_systems(&mut self, ctx: &mut Rltk) {
        self.run_state = systems::player::try_player_turn(self, ctx);
    }

    fn run_combat_systems(&mut self) {
        systems::combat::melee::MeleeCombatSystem {}.run_now(&self.ecs);
        systems::combat::damage::DamageSystem {}.run_now(&self.ecs);
        systems::combat::damage::delete_the_dead(&mut self.ecs);
    }

    fn run_ai_systems(&mut self) {
        systems::ai::AISystem {}.run_now(&self.ecs);
    }

    fn run_view_systems(&mut self) {
        systems::view_system::ViewSystem {}.run_now(&self.ecs);
        systems::view_system::ViewMemorySystem {}.run_now(&self.ecs);
    }

    fn run_map_systems(&mut self) {
        systems::map::MapIndexingSystem {}.run_now(&self.ecs);
    }

    fn draw_graphics(&self, ctx: &mut Rltk) {
        graphics::draw_map_with_fov(self, ctx);
        // graphics::draw_map_without_fov(self.current_map(), ctx);
        graphics::draw_entities(self, ctx);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.run_player_systems(ctx);
        if self.run_state == RunState::Running {
            self.run_combat_systems();
            self.run_ai_systems();
            self.run_view_systems();

            self.run_map_systems();

            console::log("END TURN ----------------------------------------".to_string());
        }
        self.ecs.maintain();

        ctx.cls();
        self.draw_graphics(ctx);
    }
}
