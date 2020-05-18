use serde::{Deserialize, Serialize};

// This should only contain the structs used by main.rs -- basically, this is to clean up the file. I hate the whole `pub` everything, though.

#[derive(Debug, Serialize, Deserialize)]
pub struct Enemizer {
    pub boss_shuffle:  String,
    pub enemy_shuffle: String,
    pub enemy_damage:  String,
    pub enemy_health:  String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bosses {
    pub eastern_palace: String,
    pub desert_palace: String,
    pub tower_of_hera: String,
    pub hyrule_castle: String,
    pub palace_of_darkness: String,
    pub swamp_palace: String,
    pub skull_woods: String,
    pub thieves_town: String,
    pub ice_palace: String,
    pub misery_mire: String,
    pub turtle_rock: String,
    pub ganons_tower_basement: String,
    pub ganons_tower_middle: String,
    pub ganons_tower_top: String,
    pub ganons_tower: String,
    pub ganon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prizes {
    pub crystals: [String; 7],
    pub pendants: [String; 3],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rupees {
    pub three_hundred_rupees: Vec<String>,
    pub one_hundred_rupees: Vec<String>,
    pub fifty_rupees: Vec<String>,
    pub twenty_rupees: Vec<String>,
    pub five_rupees: Vec<String>,
    pub one_rupee: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bomb {
    pub bomb_count: u8,
    pub bomb_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arrow {
    pub arrow_count: u8,
    pub arrow_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub item_name: String,
    pub item_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonMap {
    pub map_name: String,
    pub map_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Compass {
    pub compass_name: String,
    pub compass_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BigKey {
    pub key_name: String,
    pub key_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmallKey {
    pub key_name: String,
    pub key_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaceLog {
    // Stuff under meta and basic seed info
    pub enemizer: Enemizer,
    
    // Meta
    pub mode: String,
    pub goal: String,
    pub entry_crystals_ganon: String,
    pub entry_crystals_tower: String,
    pub item_placement: String,
    pub item_pool: String,
    pub item_functionality: String,
    pub dungeon_items: String,
    pub logic: String,
    pub accessibility: String,
    pub weapons: String,
    pub hints: String,
    pub spoilers: String,
    pub build: String,
    
    // I don't see any usefulness to the following. Not implemented.
    // tournament:
    // world_id:
    // size:
    // worlds:
    // rom_mode:

    pub waterfall_fairy: String,
    pub pyramid_fairy: String,
    
    pub turtle_rock_medallion: String,
    pub misery_mire_medallion: String,

    pub bosses: Bosses,
    pub prizes: Prizes,

    pub hearts: Vec<String>,
    pub heart_pieces: Vec<String>,
    pub sanc_heart: String,

    pub rupees: Rupees,
    
    pub bombs: Vec<Bomb>,
    pub arrows: Vec<Arrow>,

    pub maps: Vec<DungeonMap>,
    pub compasses: Vec<Compass>,
    pub big_keys: Vec<BigKey>,
    pub small_keys: Vec<SmallKey>,
    pub items: Vec<Item>,
    pub triforce_pieces: Vec<String>,

    // Currently, I'm unaware of a need to implement these.
    // Equipment
    // playthrough
    // Shops
}