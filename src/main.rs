use std::env;
use std::fs;
use std::collections::HashMap;

use serde_json::{Result, Value};

// Structs were moved to another file to keep things more readable here.
mod structs;
use structs::{Enemizer, Bosses, Prizes, Rupees, Bomb, Arrow, Item, DungeonMap, Compass, BigKey, SmallKey, RaceLog};

// This is useful for the alttpr.com randomizer, V31
// So far, I've only tested on open 7/7 with all the easiest options.
// I will need to generate other seeds for more testing.
// This is as much to learn Rust as anything else.

fn main() {
    let filename = env::args().nth(1).expect("Usage: spoiler_log_parser <spoiler log filename>");
    read_parse_output(&filename);
}

fn read_parse_output(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file!");

    let json = string_to_json(&contents).unwrap_or_else(|error| {
        panic!("Problem parsing the file: {:?}", error);
    });

    let parsed = parse_json(&json).unwrap_or_else(|error| {
        panic!("Problem creating output: {:?}", error);
    });
    // println!("output {:#?}", parsed);

    let output = serde_json::to_string(&parsed).unwrap();
    println!( "{}", output.to_string() );
}

fn string_to_json(data: &str) -> Result<serde_json::value::Value> {
    let v: Value = serde_json::from_str(data)?;
    // println!("Please call {:?}", v); // v["meta"] v["Turtle Rock"][0] etc.

    Ok(v)
}

fn insert_crystal_if_exists( label: &str, prize_name: &str, crystals: &mut [String; 7] ) {
    // This is ugly, but I think cheaper than getting the string length and calling truncate on len -2
    for token in prize_name.split(":") {
        if  "Crystal1" == token  {
            crystals[0] = String::from(label);
        } else if  "Crystal2" == token  {
            crystals[1] = String::from(label);
        } else if  "Crystal3" == token  {
            crystals[2] = String::from(label);
        } else if  "Crystal4" == token  {
            crystals[3] = String::from(label);
        } else if  "Crystal5" == token  {
            crystals[4] = String::from(label);
        } else if  "Crystal6" == token  {
            crystals[5] = String::from(label);
        } else if  "Crystal7" == token  {
            crystals[6] = String::from(label);
        }

        break;
    }
}

fn insert_pendant_if_exists( label: &str, prize_name: &str, pendants: &mut [String; 3] ) {
    // This is ugly, but I think cheaper than getting the string length and calling truncate on len -2
    for token in prize_name.split(":") {
        if  "PendantOfCourage" == token  {
            pendants[0] = String::from(label);
        } else if  "PendantOfWisdom" == token  {
            pendants[1] = String::from(label);
        } else if  "PendantOfPower" == token  {
            pendants[2] = String::from(label);
        }

        break;
    }
}

fn unbox_json_str_or_return_empty_string(serde_val_val: &serde_json::value::Value) -> String {
    let unboxed_string = serde_val_val.as_str().unwrap_or("");

    return String::from(unboxed_string);
}

fn unbox_json_str_or_return_empty_str(serde_val_val: &serde_json::value::Value) -> &str {
    return serde_val_val.as_str().unwrap_or("");
}

fn parse_json(json: &serde_json::value::Value) -> Result<RaceLog> {
    // parse things out and build :)

    // Get all the prizes. These are in order for chrystals, green pendant first for pendants.
    let mut crystals: [String; 7] = [ String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new() ];
    let mut pendants: [String; 3] = [ String::new(), String::new(), String::new() ];

    let mut prize_map:HashMap<&str, &str> = HashMap::new();
    prize_map.insert("Eastern Palace","Eastern Palace - Prize:1");
    prize_map.insert("Desert Palace","Desert Palace - Prize:1");
    prize_map.insert("Tower Of Hera","Tower of Hera - Prize:1");
    prize_map.insert("Dark Palace","Palace of Darkness - Prize:1");
    prize_map.insert("Swamp Palace","Swamp Palace - Prize:1");
    prize_map.insert("Skull Woods","Skull Woods - Prize:1");
    prize_map.insert("Thieves Town","Thieves' Town - Prize:1");
    prize_map.insert("Ice Palace","Ice Palace - Prize:1");
    prize_map.insert("Misery Mire","Misery Mire - Prize:1");
    prize_map.insert("Turtle Rock","Turtle Rock - Prize:1");

    for (key, value) in prize_map.iter() {
        insert_crystal_if_exists(
            key,
            unbox_json_str_or_return_empty_str( &json[ key ][ value ] ),
            &mut crystals
        );

        insert_pendant_if_exists(
            key,
            unbox_json_str_or_return_empty_str( &json[ key ][ value ] ),
            &mut pendants
        );
    }

    // All the other stuff
    let mut sanc_heart: String = String::new();
    let mut hearts: Vec<String> = Vec::new();
    let mut heart_pieces: Vec<String> = Vec::new();
    let mut three_hundred_rupees: Vec<String> = Vec::new();
    let mut one_hundred_rupees: Vec<String> = Vec::new();
    let mut fifty_rupees: Vec<String> = Vec::new();
    let mut twenty_rupees: Vec<String> = Vec::new();
    let mut five_rupees: Vec<String> = Vec::new();
    let mut one_rupee: Vec<String> = Vec::new();
    let mut bombs: Vec<Bomb> = Vec::new();
    let mut arrows: Vec<Arrow> = Vec::new();
    let mut maps: Vec<DungeonMap> = Vec::new();
    let mut compasses: Vec<Compass> = Vec::new();
    let mut big_keys: Vec<BigKey> = Vec::new();
    let mut small_keys: Vec<SmallKey> = Vec::new();
    let mut items: Vec<Item> = Vec::new();

    let location_map: [&str; 16] = [
        "Light World",
        "Hyrule Castle",
        "Eastern Palace",
        "Desert Palace",
        "Death Mountain",
        "Castle Tower",
        "Dark World",
        "Dark Palace",
        "Swamp Palace",
        "Skull Woods",
        "Thieves Town",
        "Ice Palace",
        "Misery Mire",
        "Turtle Rock",
        "Ganons Tower",
        "Tower Of Hera",
    ];

    for location in location_map.iter() {
        // This nonsense ensures compatible types and not a None thing. unwrap_or_else must return same type. Need delcared var because expects ref to obj
        let empty_map: serde_json::map::Map<std::string::String, serde_json::value::Value> = serde_json::map::Map::new();
        let iterable_map = json[location].as_object().unwrap_or_else( || {
            &empty_map
        });

        for (key, value) in iterable_map.iter() {
            if "PieceOfHeart:1" == value {
                heart_pieces.push( String::from(key) );
            } else if "BossHeartContainer:1" == value {
                hearts.push( String::from(key) );
            } else if "ThreeHundredRupees:1" == value {
                three_hundred_rupees.push( String::from(key) );
            } else if "OneHundredRupees:1" == value {
                one_hundred_rupees.push( String::from(key) );
            } else if "FiftyRupees:1" == value {
                fifty_rupees.push( String::from(key) );
            } else if "TwentyRupees:1" == value {
                twenty_rupees.push( String::from(key) );
            } else if "FiveRupees:1" == value {
                five_rupees.push( String::from(key) );
            } else if "OneRupee:1" == value {
                one_rupee.push( String::from(key) );
            } else if "ThreeBombs:1" == value {
                bombs.push(
                    Bomb {
                        bomb_count: 3,
                        bomb_location: String::from(key),
                    }
                );
            } else if "TenBombs:1" == value {
                bombs.push(
                    Bomb {
                        bomb_count: 10,
                        bomb_location: String::from(key),
                    }
                );
            } else if "TenArrows:1" == value {
                arrows.push(
                    Arrow {
                        arrow_count: 10,
                        arrow_location: String::from(key),
                    }
                );
            } else if "Arrow:1" == value {
                arrows.push(
                    Arrow {
                        arrow_count: 1,
                        arrow_location: String::from(key),
                    }
                );
            } else if "Arrow:1" == value {
                arrows.push(
                    Arrow {
                        arrow_count: 1,
                        arrow_location: String::from(key),
                    }
                );
            } else if unbox_json_str_or_return_empty_string(&value).starts_with("Compass") {
                compasses.push(
                    Compass {
                        compass_name: unbox_json_str_or_return_empty_string(&value),
                        compass_location: String::from(key),
                    }
                );
            } else if unbox_json_str_or_return_empty_string(&value).starts_with("Map") {
                maps.push(
                    DungeonMap {
                        map_name: unbox_json_str_or_return_empty_string(&value),
                        map_location: String::from(key),
                    }
                );
            } else if unbox_json_str_or_return_empty_string(&value).starts_with("BigKey") {
                big_keys.push(
                    BigKey {
                        key_name: unbox_json_str_or_return_empty_string(&value),
                        key_location: String::from(key),
                    }
                );
            } else if unbox_json_str_or_return_empty_string(&value).starts_with("Key") {
                small_keys.push(
                    SmallKey {
                        key_name: unbox_json_str_or_return_empty_string(&value),
                        key_location: String::from(key),
                    }
                );
            } else if "HeartContainer:1" == value {
                sanc_heart = String::from(key);
            } else if String::from( key ).ends_with("Prize:1") {
                // handled elsewhere
            } else {
                items.push(
                    Item {
                        item_name: unbox_json_str_or_return_empty_string(&value),
                        item_location: String::from(key),
                    }
                );
            }
        }
    }

    // assemble final product
    let race_log = RaceLog{
        enemizer: Enemizer {
            boss_shuffle:  unbox_json_str_or_return_empty_string( &json["meta"]["enemizer.boss_shuffle"] ),
            enemy_shuffle: unbox_json_str_or_return_empty_string( &json["meta"]["enemizer.enemy_shuffle"] ),
            enemy_damage:  unbox_json_str_or_return_empty_string( &json["meta"]["enemizer.enemy_damage"] ),
            enemy_health:  unbox_json_str_or_return_empty_string( &json["meta"]["enemizer.enemy_health"] ),
        },
        
        mode: unbox_json_str_or_return_empty_string( &json["meta"]["mode"] ),
        goal: unbox_json_str_or_return_empty_string( &json["meta"]["goal"] ),
        entry_crystals_ganon: unbox_json_str_or_return_empty_string( &json["meta"]["entry_crystals_ganon"] ),
        entry_crystals_tower: unbox_json_str_or_return_empty_string( &json["meta"]["entry_crystals_tower"] ),
        item_placement: unbox_json_str_or_return_empty_string( &json["meta"]["item_placement"] ),
        item_pool: unbox_json_str_or_return_empty_string( &json["meta"]["item_pool"] ),
        item_functionality: unbox_json_str_or_return_empty_string( &json["meta"]["item_functionality"] ),
        dungeon_items: unbox_json_str_or_return_empty_string( &json["meta"]["dungeon_items"] ),
        logic: unbox_json_str_or_return_empty_string( &json["meta"]["logic"] ),
        accessibility: unbox_json_str_or_return_empty_string( &json["meta"]["accessibility"] ),
        weapons: unbox_json_str_or_return_empty_string( &json["meta"]["weapons"] ),
        hints: unbox_json_str_or_return_empty_string( &json["meta"]["hints"] ),
        spoilers: unbox_json_str_or_return_empty_string( &json["meta"]["spoilers"] ),
        build: unbox_json_str_or_return_empty_string( &json["meta"]["build"] ),

        waterfall_fairy: unbox_json_str_or_return_empty_string( &json["Special"]["Waterfall Bottle:1"] ),
        pyramid_fairy: unbox_json_str_or_return_empty_string( &json["Special"]["Pyramid Bottle:1"] ),
        
        turtle_rock_medallion: unbox_json_str_or_return_empty_string( &json["Special"]["Turtle Rock Medallion:1"] ),
        misery_mire_medallion: unbox_json_str_or_return_empty_string( &json["Special"]["Misery Mire Medallion:1"] ),

        bosses: Bosses {
            eastern_palace: unbox_json_str_or_return_empty_string( &json["Bosses"]["Eastern Palace"] ),
            desert_palace: unbox_json_str_or_return_empty_string( &json["Bosses"]["Desert Palace"] ),
            tower_of_hera: unbox_json_str_or_return_empty_string( &json["Bosses"]["Tower Of Hera"] ),
            hyrule_castle: unbox_json_str_or_return_empty_string( &json["Bosses"]["Hyrule Castle"] ),
            palace_of_darkness: unbox_json_str_or_return_empty_string( &json["Bosses"]["Palace Of Darkness"] ),
            swamp_palace: unbox_json_str_or_return_empty_string( &json["Bosses"]["Swamp Palace"] ),
            skull_woods: unbox_json_str_or_return_empty_string( &json["Bosses"]["Skull Woods"] ),
            thieves_town: unbox_json_str_or_return_empty_string( &json["Bosses"]["Thieves Town"] ),
            ice_palace: unbox_json_str_or_return_empty_string( &json["Bosses"]["Ice Palace"] ),
            misery_mire: unbox_json_str_or_return_empty_string( &json["Bosses"]["Misery Mire"] ),
            turtle_rock: unbox_json_str_or_return_empty_string( &json["Bosses"]["Turtle Rock"] ),
            ganons_tower_basement: unbox_json_str_or_return_empty_string( &json["Bosses"]["Ganons Tower Basement"] ),
            ganons_tower_middle: unbox_json_str_or_return_empty_string( &json["Bosses"]["Ganons Tower Middle"] ),
            ganons_tower_top: unbox_json_str_or_return_empty_string( &json["Bosses"]["Ganons Tower Top"] ),
            ganons_tower: unbox_json_str_or_return_empty_string( &json["Bosses"]["Ganons Tower"] ),
            ganon: unbox_json_str_or_return_empty_string( &json["Bosses"]["Ganon"] ),
        },

        prizes: Prizes {
            crystals: crystals,
            pendants: pendants,
        },

        hearts: hearts,
        heart_pieces: heart_pieces,
        sanc_heart: sanc_heart,

        rupees: Rupees {
            three_hundred_rupees: three_hundred_rupees,
            one_hundred_rupees: one_hundred_rupees,
            fifty_rupees: fifty_rupees,
            twenty_rupees: twenty_rupees,
            five_rupees: five_rupees,
            one_rupee: one_rupee,
        },

        bombs: bombs,
        arrows: arrows,
        maps: maps,
        compasses: compasses,
        small_keys: small_keys,
        big_keys: big_keys,
        items: items,
    };
    
    Ok(race_log)
}

// Tests moved to new file :) Yay!
#[cfg(test)]
mod test_main;
