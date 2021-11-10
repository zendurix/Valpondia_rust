use crate::{levels::level::LevelType, rng};

#[derive(Clone, Debug)]
pub struct SpawnPoint {
    pub map_index: usize,
    pub entity_name: String,
}

#[derive(Clone, Debug)]
pub struct SpawnEntry {
    pub entity_name: String,
    /// inclusive range (random number, how many to spawn)
    /// items will be spawned in one place
    pub rng_range: (usize, usize),
    /// chance percentage % for this entry to be spawned, default to 100
    /// Usage examples:
    ///     SpawnEntry::new("Goblin".to_string(), 2, 6) // always spawns 2-6 goblins
    ///     SpawnEntry::new("Goblin".to_string(), 1, 2).with_chance(10) // 10% chance to spawn 1-2 goblins
    pub chance_perc: usize,
}

impl SpawnEntry {
    pub fn new(name: String, range_min: usize, range_max: usize) -> SpawnEntry {
        SpawnEntry {
            entity_name: name,
            rng_range: (range_min, range_max),
            chance_perc: 100,
        }
    }
    pub fn with_chance(mut self, chance_percent: usize) -> SpawnEntry {
        self.chance_perc = chance_percent;
        self
    }

    pub fn roll_spawn_num(&self) -> usize {
        let chance = rng::random_perc() as usize;
        if chance <= self.chance_perc {
            rng::range(self.rng_range.0 as i32, self.rng_range.1 as i32) as usize
        } else {
            0
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpawnPack {
    /// this pack will not be spawned more times than `max_spawns`
    pub max_spawns: usize,
    pub spawns_counter: usize,
    /// chance percentage % for this pack to be spawned, default to 100
    pub chance_perc: usize,
    pub entities: Vec<SpawnEntry>,
    pub min_area: usize,
}

impl Default for SpawnPack {
    fn default() -> SpawnPack {
        SpawnPack {
            chance_perc: 100,
            entities: vec![],
            min_area: 0,
            max_spawns: usize::MAX,
            spawns_counter: 0,
        }
    }
}

impl SpawnPack {
    pub fn with_chance_perc(mut self, chance_perc: usize) -> SpawnPack {
        self.chance_perc = chance_perc;
        self
    }
    pub fn with_max_spawns(mut self, max_spawns: usize) -> SpawnPack {
        self.max_spawns = max_spawns;
        self
    }

    pub fn goblins_pack() -> SpawnPack {
        SpawnPack {
            min_area: 8,
            entities: vec![
                SpawnEntry::new("Goblin".to_string(), 2, 6),
                SpawnEntry::new("Health potion".to_string(), 0, 1),
                SpawnEntry::new("Sleep scrool".to_string(), 0, 1),
            ],
            ..SpawnPack::default()
        }
    }

    pub fn orcs_pack() -> SpawnPack {
        SpawnPack {
            min_area: 4,
            entities: vec![
                SpawnEntry::new("Orc".to_string(), 1, 3),
                SpawnEntry::new("Health potion".to_string(), 1, 2),
                SpawnEntry::new("Magic missile scrool".to_string(), 1, 1).with_chance(50),
            ],
            ..SpawnPack::default()
        }
    }

    pub fn goblins_with_orc_pack() -> SpawnPack {
        SpawnPack {
            min_area: 5,
            entities: vec![
                SpawnEntry::new("Orc".to_string(), 1, 1),
                SpawnEntry::new("Goblin".to_string(), 2, 4),
                SpawnEntry::new("Health potion".to_string(), 1, 2),
                SpawnEntry::new("Great health potion".to_string(), 0, 1).with_chance(20),
                SpawnEntry::new("Magic missile scrool".to_string(), 1, 1).with_chance(70),
                SpawnEntry::new("Sleep scrool".to_string(), 1, 1).with_chance(30),
            ],
            ..SpawnPack::default()
        }
    }

    pub fn knight_pack() -> SpawnPack {
        SpawnPack {
            min_area: 5,
            entities: vec![
                SpawnEntry::new("Knight".to_string(), 1, 1),
                SpawnEntry::new("Health potion".to_string(), 2, 3),
                SpawnEntry::new("Great health potion".to_string(), 0, 1).with_chance(80),
                SpawnEntry::new("Fireball scrool".to_string(), 1, 1).with_chance(70),
                SpawnEntry::new("Teleport scrool".to_string(), 1, 1).with_chance(45),
            ],
            ..SpawnPack::default()
        }
    }

    pub fn humans_pack() -> SpawnPack {
        SpawnPack {
            min_area: 5,
            entities: vec![
                SpawnEntry::new("Human".to_string(), 2, 4),
                SpawnEntry::new("Great health potion".to_string(), 2, 3).with_chance(90),
                SpawnEntry::new("Fireball scrool".to_string(), 1, 2).with_chance(70),
                SpawnEntry::new("Teleport scrool".to_string(), 1, 1).with_chance(70),
            ],
            ..SpawnPack::default()
        }
    }
}

pub struct SpawnTable {
    pub level_type: LevelType,
    pub weight: usize,
    pub spawn_packs: Vec<SpawnPack>,
}

impl SpawnTable {
    /// returns None, if no spawn can happen for this area.
    pub fn roll_spawn_pack_index(&mut self, area: usize) -> Option<usize> {
        let max_rolls = 100;
        let mut i = 0;
        loop {
            if i >= max_rolls {
                println!("Cannot spawn for this area: {}", area);
                return None;
            }

            let index = rng::range(0, self.spawn_packs.len() as i32 - 1) as usize;
            let chance = rng::random_perc() as usize;

            if chance <= self.spawn_packs[index].chance_perc
                && self.spawn_packs[index].spawns_counter < self.spawn_packs[index].max_spawns
                && self.spawn_packs[index].min_area <= area
            {
                self.spawn_packs[index].spawns_counter += 1;
                return Some(index);
            }

            i += 1;
        }
    }

    pub fn basic_dungeon() -> SpawnTable {
        SpawnTable {
            level_type: LevelType::BasicDungeon,
            weight: 0,
            spawn_packs: vec![
                SpawnPack::goblins_pack().with_max_spawns(5),
                SpawnPack::orcs_pack().with_max_spawns(2),
                SpawnPack::goblins_with_orc_pack().with_max_spawns(1),
                SpawnPack::knight_pack().with_max_spawns(1),
                SpawnPack::humans_pack()
                    .with_max_spawns(1)
                    .with_chance_perc(30),
            ],
        }
    }

    pub fn caves() -> SpawnTable {
        SpawnTable {
            level_type: LevelType::Cave,
            weight: 0,
            spawn_packs: vec![
                SpawnPack::goblins_pack().with_max_spawns(10),
                SpawnPack::orcs_pack().with_max_spawns(2),
            ],
        }
    }
}
