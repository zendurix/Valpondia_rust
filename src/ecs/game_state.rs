use lazy_static::__Deref;
use rltk::{DrawBatch, GameState, Point, Rltk};
use specs::prelude::*;

use crate::ecs::components;
use crate::ecs::errors::Result;
use crate::ecs::systems;
use crate::graphics::gui::menus::main_menu::MainMenu;
use crate::graphics::gui::menus::WindowOptionSelector;
use crate::graphics::gui::{
    EquipmentMenuAction, InventoryMenuAction, ItemMenuAction, PopuSelection, TargetingMenuAction,
};
use crate::graphics::window::{
    CHAR_CONSOLE_INDEX, SPRITE_16X16_CONSOLE_INDEX, SPRITE_32X32_CONSOLE_INDEX,
};
use crate::graphics::{self, gui, GuiDrawer};
use crate::levels::level::{Level, LevelType};
use crate::levels::level_manager::LevelManager;
use crate::maps::{Map, TileType};
use crate::rng;
use crate::spawner::player::spawn_player;
use crate::spawner::spawn_from_spawn_table;
use crate::spawner::spawn_tables::SpawnTable;

#[cfg(feature = "map_gen_testing")]
use crate::graphics::gui::menus::map_testing::GuiMapGenTestingManager;
#[cfg(feature = "map_gen_testing")]
use crate::maps::generators::basic_dungeon::{BasicDungeonMap, BasicDungeonMapConfig};
#[cfg(feature = "map_gen_testing")]
use crate::maps::generators::cellular_automata::CAMapGen;
#[cfg(feature = "map_gen_testing")]
use gui::menus::map_testing::MapGenTestingMenuAction;

use super::components::BodyPart;

pub struct GameLog {
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetingAction {
    TargetingFromItem(Entity, usize),
    Looking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    MainMenu,
    #[cfg(feature = "map_gen_testing")]
    MapGenTesting(bool),
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowEquipment,
    ShowItemActions(Entity),
    Targeting(TargetingAction),
    MoveLevel(usize),

    GameOver,
    Controls,
    Won,
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

    pub level_order: Vec<LevelType>,

    pub game_won: bool,
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
            level_order: vec![
                LevelType::BasicDungeon,
                LevelType::BSPDungeon,
                LevelType::Cave,
                LevelType::DrunkardWalk,
                LevelType::BSPInterior,
                LevelType::TestLevel,
            ],
            game_won: false,
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
        self.ecs.register::<components::SpawnsAfterDeath>();
        self.ecs.register::<components::Spawn>();
        self.ecs.register::<components::Equippable>();
        self.ecs.register::<components::Equipped>();
        self.ecs.register::<components::WantsToEquip>();
        self.ecs.register::<components::WantsToUnEquip>();
        self.ecs.register::<components::BodyParts>();
        self.ecs.register::<components::MeleeDamageBonus>();
        self.ecs.register::<components::DefenseBonus>();
        self.ecs.register::<components::Inventory>();
        self.ecs.register::<components::FinalBoss>();
    }

    pub fn reset_gui_inv_manager(&mut self) {
        let mut inv_manager = self.gui_drawer.inv_manager.clone();
        inv_manager.reset(self);
        self.gui_drawer.inv_manager = inv_manager;
    }

    pub fn reset_gui_eq_manager(&mut self) {
        let mut eq_manager = self.gui_drawer.eq_manager.clone();
        eq_manager.reset(self);
        self.gui_drawer.eq_manager = eq_manager;
    }

    pub fn reset_gui_item_action_manager(&mut self, item: Entity) {
        let mut item_action_manager = self.gui_drawer.item_action_manager.clone();
        item_action_manager.reset(self, item);
        self.gui_drawer.item_action_manager = item_action_manager;
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

        let spawn_table = match level_type {
            LevelType::Cave => SpawnTable::caves(),
            LevelType::BasicDungeon => SpawnTable::basic_dungeon(),
            LevelType::BSPDungeon => SpawnTable::bsp_dungeon(),
            LevelType::BSPInterior => SpawnTable::bsp_interior(),
            LevelType::DrunkardWalk => SpawnTable::drunkard_walk(),
            LevelType::TestLevel => SpawnTable::boss_level(),
        };

        spawn_from_spawn_table(
            &mut self.ecs,
            &self.level_manager.levels[index],
            spawn_table,
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

            let new_level_type = self.level_order[next_level];

            let new_level_index = self
                .create_new_level(
                    new_level_type,
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
        systems::combat::drop_after_death::DropAfterDeathSystem {}.run_now(&self.ecs);
        systems::inventory::ItemDropSystem {}.run_now(&self.ecs);
        systems::inventory::UseItemSystem {}.run_now(&self.ecs);
        systems::inventory::DestroyUsedItems {}.run_now(&self.ecs);

        systems::inventory::ItemEquipSystem {}.run_now(&self.ecs);
        systems::inventory::ItemUnEquipSystem {}.run_now(&self.ecs);
    }

    fn run_effects_systems(&mut self) {
        systems::effects::HealSystem {}.run_now(&self.ecs);
        systems::effects::TeleportSystem {}.run_now(&self.ecs);
        systems::combat::spawn_after_death::SpawnsAfterDeathSystem {}.run_now(&self.ecs);
        systems::spawn::spawn_system(self);
    }

    fn run_all_gameplay_systems(&mut self) {
        self.run_ai_systems();
        self.run_combat_systems();
        self.run_inventory_systems();
        self.run_effects_systems();

        systems::combat::damage::delete_the_dead(self);
        self.run_view_systems();
        self.run_map_systems();
    }

    fn draw_game_graphics(&self, ctx: &mut Rltk) {
        graphics::draw_map_and_entities_with_fov_and_camera(self, ctx);
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

    fn equip_item(&mut self, item: Entity, target_body_part: BodyPart) {
        let mut wants_eq = self.ecs.write_storage::<components::WantsToEquip>();
        let player = *self.ecs.fetch::<Entity>();
        wants_eq
            .insert(
                player,
                components::WantsToEquip {
                    item,
                    target_body_part,
                },
            )
            .expect("Unable to insert intent to equip item");
    }

    fn unequip_item(&mut self, item: Entity) {
        let mut wants_uneq = self.ecs.write_storage::<components::WantsToUnEquip>();
        let player = *self.ecs.fetch::<Entity>();
        wants_uneq
            .insert(player, components::WantsToUnEquip { item })
            .expect("Unable to insert intent to unequip item");
    }

    fn drop_item(&mut self, item: Entity) {
        let mut items_drops = self.ecs.write_storage::<components::WantsToDropItem>();
        let player = *self.ecs.fetch::<Entity>();
        items_drops
            .insert(player, components::WantsToDropItem { item })
            .expect("Unable to insert intent to drop item");
    }

    fn game_over_cleanup(&mut self) {
        // Delete everything
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            self.ecs.delete_entity(*del).expect("Deletion failed");
        }

        let new_gamelog = GameLog {
            entries: vec![
                "  =====WELCOME INTO ROGUELIKE======  ".to_string(),
                "Your task is to delve deep into dungeon and slain Mighty Slime ".to_string(),
            ],
        };
        self.ecs.remove::<Level>();
        self.ecs.insert(new_gamelog);

        // Build a new map and place the player
        self.level_manager.reset();
        let test = self.create_new_level(
            self.level_order[0],
            self.map_width,
            self.map_height,
            0,
            None,
        );

        match test {
            Ok(_) => (),
            Err(e) => {
                println!("ERROR: {}", e);
                std::process::exit(1);
            }
        }
        self.set_level_as_curent(0);

        let mut p_x = 0;
        let mut p_y = 0;
        while self.current_map().tile_at_xy(p_x, p_y).blocks_movement() {
            p_x = rng::range(2, self.current_map().width_max() as i32 - 2) as usize;
            p_y = rng::range(2, self.current_map().height_max() as i32 - 2) as usize;
        }

        self.ecs.insert(rltk::Point::new(p_x, p_y));
        let player = spawn_player(&mut self.ecs, p_x, p_y);

        self.ecs.insert(player);
    }

    pub fn draw_graphics(&self, ctx: &mut Rltk) {
        let run_state = *self.ecs.fetch::<RunState>();

        ctx.set_active_console(CHAR_CONSOLE_INDEX);
        ctx.cls();
        ctx.set_active_console(SPRITE_16X16_CONSOLE_INDEX);
        ctx.cls();
        ctx.set_active_console(SPRITE_32X32_CONSOLE_INDEX);
        ctx.cls();

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(SPRITE_16X16_CONSOLE_INDEX);
        draw_batch.cls();
        draw_batch.target(SPRITE_32X32_CONSOLE_INDEX);
        draw_batch.cls();

        match run_state {
            RunState::MainMenu | RunState::Controls => {}
            #[cfg(feature = "map_gen_testing")]
            RunState::MapGenTesting(_) => {}
            _ => {
                self.draw_game_graphics(ctx);
            }
        }

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(SPRITE_16X16_CONSOLE_INDEX);
        draw_batch
            .submit(SPRITE_16X16_CONSOLE_INDEX)
            .expect("Draw Batch error");
        rltk::render_draw_buffer(ctx).expect("Render error");

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(SPRITE_32X32_CONSOLE_INDEX);
        draw_batch
            .submit(SPRITE_32X32_CONSOLE_INDEX)
            .expect("Draw Batch error");
        rltk::render_draw_buffer(ctx).expect("Render error");

        match run_state {
            RunState::ShowInventory => self.gui_drawer.inv_manager.draw(ctx),

            RunState::ShowEquipment => {
                self.gui_drawer.eq_manager.draw(ctx);
            }

            RunState::ShowItemActions(_item) => {
                self.gui_drawer.item_action_manager.draw(ctx);
            }

            _ => {}
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut run_state = *self.ecs.fetch::<RunState>();

        self.draw_graphics(ctx);

        match run_state {
            RunState::PreRun => {
                self.run_all_gameplay_systems();
                run_state = RunState::AwaitingInput;
            }

            RunState::AwaitingInput => {
                run_state = systems::player::try_player_turn(self, ctx);
            }

            RunState::ShowInventory => {
                let inv_action = self.gui_drawer.inv_manager.update(ctx);
                match inv_action {
                    InventoryMenuAction::NoResponse => (),
                    InventoryMenuAction::Cancel => run_state = RunState::AwaitingInput,
                    InventoryMenuAction::SelectedItem(item) => {
                        self.reset_gui_item_action_manager(item);
                        run_state = RunState::ShowItemActions(item)
                    }
                }
            }

            RunState::ShowEquipment => {
                let eq_action = self.gui_drawer.eq_manager.update(ctx);
                match eq_action {
                    EquipmentMenuAction::NoResponse => (),
                    EquipmentMenuAction::Cancel => run_state = RunState::AwaitingInput,
                }
            }

            RunState::ShowItemActions(item) => {
                let item_action = self.gui_drawer.item_action_manager.update(ctx, item);
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
                    ItemMenuAction::Equip(item) => {
                        // TODO add limb selection menu here
                        let target;
                        {
                            let equipables = self.ecs.read_storage::<components::Equippable>();
                            let equipable = equipables.get(item).unwrap();
                            target = if equipable.body_part == BodyPart::OneHanded {
                                BodyPart::HandRight
                            } else {
                                equipable.body_part
                            };
                        }

                        self.equip_item(item, target);
                        run_state = RunState::PlayerTurn;
                    }
                    ItemMenuAction::UnEquip(item) => {
                        self.unequip_item(item);
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
                let run_state_check = *self.ecs.fetch::<RunState>();

                if run_state_check != RunState::GameOver {
                    run_state = RunState::MonsterTurn;
                } else {
                    run_state = RunState::GameOver;
                }
            }

            RunState::MonsterTurn => {
                self.run_all_gameplay_systems();
                let run_state_check = *self.ecs.fetch::<RunState>();
                if run_state_check != RunState::GameOver {
                    run_state = RunState::AwaitingInput;
                } else {
                    run_state = RunState::GameOver;
                }
            }

            RunState::MainMenu => {
                let main_menu_action = self.main_menu.draw(ctx);
                match main_menu_action {
                    gui::MainMenuAction::NotSelected => {
                        run_state = RunState::MainMenu;
                    }
                    gui::MainMenuAction::Selected(selected) => match selected {
                        gui::MainMenuSelection::NewGame => {
                            run_state = {
                                self.game_over_cleanup();
                                RunState::PreRun
                            }
                        }
                        #[cfg(feature = "map_gen_testing")]
                        gui::MainMenuSelection::MapGenTesting => {
                            self.gui_drawer.map_gen_testing_manager.reset();
                            run_state = RunState::MapGenTesting(false)
                        }
                        gui::MainMenuSelection::Controls => run_state = RunState::Controls,
                        gui::MainMenuSelection::Quit => {
                            std::process::exit(0);
                        }
                    },
                }
            }

            RunState::MoveLevel(next_level) => {
                if next_level == self.level_order.len() {
                    if self.game_won {
                        run_state = RunState::Won;
                    } else {
                        let mut gamelog = self.ecs.write_resource::<GameLog>();
                        gamelog
                            .entries
                            .push("You must kill the Mighty Slime to finish the game!".to_string());
                        run_state = RunState::PlayerTurn;

                        let mut positions = self.ecs.write_storage::<components::Position>();
                        let player = self.ecs.fetch::<Entity>();
                        let mut pos = positions.get_mut(*player).unwrap();
                        pos.level -= 1;
                    }
                } else {
                    self.player_move_level(next_level);
                    run_state = RunState::PreRun;
                }
            }

            RunState::GameOver => {
                let result = self.gui_drawer.game_over(ctx);
                match result {
                    PopuSelection::NoSelection => {}
                    PopuSelection::QuitToMenu => {
                        self.game_over_cleanup();
                        run_state = RunState::MainMenu;
                    }
                }
            }

            RunState::Won => {
                let result = self.gui_drawer.game_won(ctx);
                match result {
                    PopuSelection::NoSelection => {}
                    PopuSelection::QuitToMenu => {
                        self.game_over_cleanup();
                        run_state = RunState::MainMenu;
                    }
                }
            }

            RunState::Controls => {
                let result = self.gui_drawer.show_controls(ctx);
                match result {
                    PopuSelection::NoSelection => {}
                    PopuSelection::QuitToMenu => {
                        run_state = RunState::MainMenu;
                    }
                }
            }

            #[cfg(feature = "map_gen_testing")]
            RunState::MapGenTesting(draw_map) => {
                if draw_map {
                    run_state = print_tested_map(
                        &mut self.gui_drawer.map_gen_testing_manager,
                        ctx,
                        self.window_height,
                    );
                } else {
                    run_state = print_map_testing_menu(self, ctx);
                }
            }
        }
        *self.ecs.write_resource::<RunState>() = run_state;

        self.ecs.maintain();
    }
}

#[cfg(feature = "map_gen_testing")]
fn print_map_testing_menu(state: &mut State, ctx: &mut Rltk) -> RunState {
    use crate::maps::generators::{
        bsp::{interior::BSPInteriorGen, BSPConfig, BSPDungeonGen},
        drunkard_walk::{DrunkardWalkConfig, DrunkardWalkGen},
    };

    let mut run_state = RunState::MapGenTesting(false);

    let map_testing_action = state.gui_drawer.map_gen_testing_manager.update(ctx);
    match map_testing_action {
        MapGenTestingMenuAction::NoResponse => (),
        MapGenTestingMenuAction::Cancel => run_state = RunState::MainMenu,
        MapGenTestingMenuAction::SwitchShowSteps => {
            state.gui_drawer.map_gen_testing_manager.switch_show_steps();
            state.gui_drawer.map_gen_testing_manager.reset();
        }
        MapGenTestingMenuAction::TestBasicDungeonGenerator => {
            state
                .gui_drawer
                .map_gen_testing_manager
                .reset_map_gen(Box::new(BasicDungeonMap::new(
                    state.window_width - 4,
                    state.map_height - 4,
                    BasicDungeonMapConfig::default(),
                )));
            run_state = RunState::MapGenTesting(true);
        }
        MapGenTestingMenuAction::TestCaMapGen => {
            state
                .gui_drawer
                .map_gen_testing_manager
                .reset_map_gen(Box::new(
                    CAMapGen::new(state.window_width - 4, state.map_height - 4).unwrap(),
                ));
            run_state = RunState::MapGenTesting(true);
        }
        MapGenTestingMenuAction::TestBSPDungeonGen => {
            state
                .gui_drawer
                .map_gen_testing_manager
                .reset_map_gen(Box::new(BSPDungeonGen::new(
                    state.window_width - 4,
                    state.map_height - 4,
                    BSPConfig::default(),
                )));
            run_state = RunState::MapGenTesting(true);
        }
        MapGenTestingMenuAction::TestBSPInteriorGen => {
            state
                .gui_drawer
                .map_gen_testing_manager
                .reset_map_gen(Box::new(BSPInteriorGen::new(
                    state.window_width - 4,
                    state.map_height - 4,
                    BSPConfig::default(),
                )));
            run_state = RunState::MapGenTesting(true);
        }
        MapGenTestingMenuAction::TestDrunkardWalkGen => {
            state
                .gui_drawer
                .map_gen_testing_manager
                .reset_map_gen(Box::new(DrunkardWalkGen::new(
                    state.window_width - 4,
                    state.map_height - 4,
                    DrunkardWalkConfig::default(),
                )));
            run_state = RunState::MapGenTesting(true);
        }
    }

    run_state
}

#[cfg(feature = "map_gen_testing")]
fn print_tested_map(
    manager: &mut GuiMapGenTestingManager,
    ctx: &mut Rltk,
    window_height: usize,
) -> RunState {
    use crate::{
        ecs::systems::player::{input::get_input, InputType},
        graphics::draw_map_without_fov,
    };

    ctx.set_active_console(CHAR_CONSOLE_INDEX);
    ctx.cls();

    let history = manager.map_gen.try_get_history();
    let history_size = history.len();

    let mut current_index = manager.current_history_index;
    if current_index == history_size {
        manager.reset_current_map_gen();
        return RunState::MapGenTesting(true);
    }

    if !manager.show_steps {
        current_index = history_size - 1;
        manager.current_history_index = current_index;
    }

    draw_map_without_fov(&history[current_index].0, ctx);

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(SPRITE_16X16_CONSOLE_INDEX);
    draw_batch
        .submit(SPRITE_16X16_CONSOLE_INDEX)
        .expect("Draw Batch error");
    rltk::render_draw_buffer(ctx).expect("Render error");

    ctx.set_active_console(CHAR_CONSOLE_INDEX);
    let press_enter_info = if current_index < history_size - 1 {
        "Press Spacebar to progres step"
    } else {
        " ---- Generating Map Done. Press Spacebar generate new map. ----"
    };

    ctx.print_color(
        1,
        window_height - 4,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        format!("Current Step: {} ", current_index),
    );

    ctx.print_color(
        1,
        window_height - 3,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        format!("STEP: {} ", &history[current_index].1),
    );

    ctx.print_color(
        1,
        window_height - 2,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        press_enter_info,
    );

    ctx.print_color(
        1,
        window_height - 1,
        rltk::RGB::named(rltk::WHITE),
        rltk::RGB::named(rltk::BLACK),
        "Press EESCAPE to return to menu",
    );

    let input = get_input(ctx);
    if let Some(key) = input {
        match key {
            InputType::Spacebar => {
                manager.current_history_index += 1;
                RunState::MapGenTesting(true)
            }
            InputType::Escape => RunState::MapGenTesting(false),
            _ => RunState::MapGenTesting(true),
        }
    } else {
        RunState::MapGenTesting(true)
    }
}
