use lazy_static::__Deref;
use rltk::{GameState, Point, Rltk};
use specs::prelude::*;

use crate::ecs::components;
use crate::ecs::errors::Result;
use crate::ecs::systems;
use crate::graphics::gui::menus::main_menu::MainMenu;
use crate::graphics::gui::{InventoryMenuAction, ItemMenuAction, TargetingMenuAction};
use crate::graphics::{self, gui, GuiDrawer};
use crate::levels::level::{Level, LevelType};
use crate::levels::level_manager::LevelManager;
use crate::maps::{Map, TileType};
use crate::spawner::spawn_from_spawn_table;
use crate::spawner::spawn_tables::SpawnTable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetingAction {
    TargetingFromItem(Entity, usize),
    Looking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    MainMenu,
    SaveGame,
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowItemActions(Entity),
    Targeting(TargetingAction),
    MoveLevel(usize),
}

/// State global resources (stored in rltk)
/// rltk::Point - player position
/// Entity - player entity (id)
/// Level - current level
/// RunState - run state
/// GameLog - messages log

pub struct State {
    pub ecs: World,

    pub gui_drawer: GuiDrawer,
    pub main_menu: MainMenu,

    pub level_manager: LevelManager,
    pub current_level: usize,

    pub window_width: usize,
    pub window_height: usize,

    pub map_width: usize,
    pub map_height: usize,

    pub targeting_pos: Point,
}

impl State {
    pub fn new(
        window_width: usize,
        window_height: usize,
        map_width: usize,
        map_height: usize,
        gui_drawer: GuiDrawer,
    ) -> State {
        State {
            current_level: 0,
            ecs: World::new(),
            level_manager: LevelManager::new(),
            window_width,
            window_height,
            map_width,
            map_height,
            gui_drawer,
            main_menu: MainMenu::new(),
            targeting_pos: Point::new(0, 0),
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
        self.ecs.register::<components::Item>();
        self.ecs.register::<components::Heal>();
        self.ecs.register::<components::InInventory>();
        self.ecs.register::<components::WantsToPickupItem>();
        self.ecs.register::<components::WantsToUseItem>();
        self.ecs.register::<components::WantsToDropItem>();
        self.ecs.register::<components::Usable>();
        self.ecs.register::<components::Ranged>();
        self.ecs.register::<components::InflictsDamage>();
        self.ecs.register::<components::AreaOfEffect>();
        self.ecs.register::<components::SleepingEffect>();
        self.ecs.register::<components::Sleeping>();
        self.ecs.register::<components::HealEffect>();
        self.ecs.register::<components::Teleporting>();
        self.ecs.register::<components::TeleportingEffect>();
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
        depth: usize,
        prev_down_stairs_pos: Option<(usize, usize)>,
    ) -> Result<usize> {
        let index = self.level_manager.crete_new_level(
            level_type,
            width,
            height,
            depth,
            prev_down_stairs_pos.map(|pos| Point::new(pos.0, pos.1)),
        )?;
        spawn_from_spawn_table(
            &mut self.ecs,
            &self.level_manager.levels[index],
            SpawnTable::first_level(),
        );
        Ok(index)
    }

    pub fn set_level_as_curent(&mut self, level_index: usize) {
        // TODO level_index to high error

        if let Some(current_level) = self.ecs.try_fetch::<Level>() {
            let level = current_level.deref().clone();
            self.level_manager.levels[self.current_level] = level;
        }

        self.current_level = level_index;
        self.ecs
            .insert(self.level_manager.levels[level_index].clone());
    }

    pub fn player_move_level(&mut self, next_level: usize) {
        if next_level < self.level_manager.levels.len() {
            self.set_level_as_curent(next_level);
        } else {
            let mut current_depth = None;
            let mut prev_down_stairs_pos = None;
            if let Some(current_level) = self.ecs.try_fetch::<Level>() {
                current_depth = Some(current_level.depth);
                prev_down_stairs_pos = current_level
                    .map
                    .tiles
                    .iter()
                    .enumerate()
                    .find_map(|(i, t)| (*t == TileType::StairsDown).then(|| i))
                    .map(|i| current_level.map.index_to_xy(i));
            }

            let new_level_index = self
                .create_new_level(
                    LevelType::BasicDungeon,
                    self.map_width,
                    self.map_height,
                    current_depth.map(|d| d + 1).unwrap_or(0),
                    prev_down_stairs_pos,
                )
                .unwrap();
            self.set_level_as_curent(new_level_index);
        }
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

    fn run_inventory_systems(&mut self) {
        systems::inventory::ItemCollectionSystem {}.run_now(&self.ecs);
        systems::inventory::ItemDropSystem {}.run_now(&self.ecs);
        systems::inventory::UseItemSystem {}.run_now(&self.ecs);
        systems::inventory::DestroyUsedItems {}.run_now(&self.ecs);
    }

    fn run_effects_systems(&mut self) {
        systems::effects::HealSystem {}.run_now(&self.ecs);
        systems::effects::TeleportSystem {}.run_now(&self.ecs);
    }

    fn run_all_gameplay_systems(&mut self) {
        self.run_ai_systems();
        self.run_inventory_systems();
        self.run_combat_systems();
        self.run_effects_systems();
        self.run_view_systems();
        self.run_map_systems();
    }

    fn draw_game_graphics(&self, ctx: &mut Rltk) {
        graphics::draw_map_with_fov(self, ctx);
        // graphics::draw_map_without_fov(self.current_map(), ctx);
        graphics::draw_entities(self, ctx);
        self.gui_drawer.draw_ui(&self.ecs, ctx);
    }

    fn use_item(&mut self, item: Entity, targeted: bool) {
        let mut items_uses = self.ecs.write_storage::<components::WantsToUseItem>();
        let player = *self.ecs.fetch::<Entity>();
        items_uses
            .insert(
                player,
                components::WantsToUseItem {
                    item,
                    target: targeted.then(|| self.targeting_pos),
                },
            )
            .expect("Unable to insert intent to use item");
    }

    fn drop_item(&mut self, item: Entity) {
        let mut items_drops = self.ecs.write_storage::<components::WantsToDropItem>();
        let player = *self.ecs.fetch::<Entity>();
        items_drops
            .insert(player, components::WantsToDropItem { item })
            .expect("Unable to insert intent to drop item");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut run_state = *self.ecs.fetch::<RunState>();
        ctx.cls();

        match run_state {
            RunState::MainMenu => {}
            _ => self.draw_game_graphics(ctx),
        }

        match run_state {
            RunState::PreRun => {
                self.run_all_gameplay_systems();
                run_state = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                run_state = systems::player::try_player_turn(self, ctx);
            }
            RunState::ShowInventory => {
                let inv_action = gui::show_inventory(self, ctx);
                match inv_action {
                    InventoryMenuAction::NoResponse => (),
                    InventoryMenuAction::Cancel => run_state = RunState::AwaitingInput,
                    InventoryMenuAction::SelectedItem(item) => {
                        run_state = RunState::ShowItemActions(item)
                    }
                }
            }
            RunState::ShowItemActions(item) => {
                let item_action = gui::show_item_actions(self, ctx, item);
                match item_action {
                    ItemMenuAction::Cancel => run_state = RunState::ShowInventory,
                    ItemMenuAction::NoResponse => (),
                    ItemMenuAction::Use(item) => {
                        let mut range = 0;
                        let mut is_ranged = false;
                        {
                            let ranged = self.ecs.read_storage::<components::Ranged>();
                            if let Some(r) = ranged.get(item) {
                                is_ranged = true;
                                range = r.range;
                            }
                        }
                        if is_ranged {
                            let player_pos = self.ecs.fetch::<rltk::Point>();
                            self.targeting_pos = *player_pos;
                            run_state = RunState::Targeting(TargetingAction::TargetingFromItem(
                                item,
                                range as usize,
                            ));
                        } else {
                            self.use_item(item, false);
                            run_state = RunState::PlayerTurn;
                        }
                    }
                    ItemMenuAction::Drop(item) => {
                        self.drop_item(item);
                        run_state = RunState::PlayerTurn;
                    }
                }
            }
            RunState::Targeting(action) => {
                let target_menu_action = gui::show_targeting(self, ctx, action);
                match target_menu_action {
                    TargetingMenuAction::Cancel => run_state = RunState::AwaitingInput,
                    TargetingMenuAction::NoResponse => (),
                    TargetingMenuAction::Selected => match action {
                        TargetingAction::TargetingFromItem(item, _range) => {
                            self.use_item(item, true);
                            run_state = RunState::PlayerTurn;
                        }
                        TargetingAction::Looking => (),
                    },
                }
            }
            RunState::PlayerTurn => {
                self.run_all_gameplay_systems();
                run_state = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_all_gameplay_systems();
                run_state = RunState::AwaitingInput;
            }

            RunState::MainMenu => {
                let main_menu_action = self.main_menu.draw(ctx);
                match main_menu_action {
                    gui::MainMenuAction::NotSelected => {
                        run_state = RunState::MainMenu;
                    }
                    gui::MainMenuAction::Selected(selected) => match selected {
                        gui::MainMenuSelection::NewGame => run_state = RunState::PreRun,
                        gui::MainMenuSelection::LoadGame => run_state = RunState::PreRun,

                        // not implemented
                        gui::MainMenuSelection::Credits => run_state = RunState::PreRun,
                        gui::MainMenuSelection::Quit => {
                            std::process::exit(0);
                        }
                    },
                }
            }
            RunState::SaveGame => {
                //  let data = serde_json::to_string(&self.ecs.fetch::<Level>().map).unwrap();
                //  println!("{}", data);
                run_state = RunState::MainMenu;
            }
            RunState::MoveLevel(next_level) => {
                self.player_move_level(next_level);
                run_state = RunState::PreRun;
            }
        }
        *self.ecs.write_resource::<RunState>() = run_state;

        self.ecs.maintain();
    }
}
