use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use serde_json::{Result, Value};

// Structs were moved to another file to keep things more readable here.
mod structs;
use structs::{Enemizer, Bosses, Prizes, Rupees, Bomb, Arrow, Item, DungeonMap, Compass, BigKey, SmallKey, RaceLog};

// This is useful for the alttpr.com randomizer, V31
// So far, I've only tested on open 7/7 with all the easiest options.
// I will need to generate other seeds for more testing.
// This is as much to learn Rust as anything else.
//
// TODO: get confirmation on the meanings of some of the things, especially in "meta"
// TODO: clean up some stuff when I'm better at rust :P :)

fn main() {
    let filename = env::args().nth(1).expect("Usage: spoiler_log_parser <spoiler log filename> <output filename>");
    let outfile = env::args().nth(2).expect("Usage: spoiler_log_parser <spoiler log filename> <output filename>");
    println!("Reading from file: {}", filename);
    read_parse_output(&filename, &outfile);
}

fn read_parse_output(filename: &str, outfile: &str) {
    let contents = fs::read_to_string(filename).expect("Could not read file!");

    let json = string_to_json(&contents).unwrap_or_else(|error| {
        panic!("Problem parsing the file: {:?}", error);
    });

    let parsed = parse_json(&json).unwrap_or_else(|error| {
        panic!("Problem creating output: {:?}", error);
    });
    // println!("output {:#?}", parsed);

    let output = serde_json::to_string(&parsed).unwrap();
    // println!("output {:#?}", output);

    let mut file = File::create(&outfile).unwrap_or_else(|error| {
        panic!("Could not create file: {:?}", error);
    });

    let _write_result = file.write_all(&output.as_bytes()).unwrap_or_else(|error| {
        panic!("Could not write file: {:?}", error);
    });   

    println!("Output file written. Enjoy!");
}

fn string_to_json(data: &str) -> Result<serde_json::value::Value> {
    let v: Value = serde_json::from_str(data)?;
    // println!("Please call {:?}", v); // v["meta"] v["Turtle Rock"][0] etc.

    Ok(v)
}

fn insert_crystal_if_exists( label: String, prize_name: String, crystals: &mut [String; 7] ) {
    // This is ugly. Is there not a better way in rust?
    for token in prize_name.split(":") {
        if  "Crystal1" == token  {
            crystals[0] = label;
        } else if  "Crystal2" == token  {
            crystals[1] = label;
        } else if  "Crystal3" == token  {
            crystals[2] = label;
        } else if  "Crystal4" == token  {
            crystals[3] = label;
        } else if  "Crystal5" == token  {
            crystals[4] = label;
        } else if  "Crystal6" == token  {
            crystals[5] = label;
        } else if  "Crystal7" == token  {
            crystals[6] = label;
        }

        break;
    }
}

fn insert_pendant_if_exists( label: String, prize_name: String, pendants: &mut [String; 3] ) {
    // This is ugly. Is there not a better way in rust?
    for token in prize_name.split(":") {
        if  "PendantOfCourage" == token  {
            pendants[0] = label;
        } else if  "PendantOfWisdom" == token  {
            pendants[1] = label;
        } else if  "PendantOfPower" == token  {
            pendants[2] = label;
        }

        break;
    }
}

fn parse_json(json: &serde_json::value::Value) -> Result<RaceLog> {
    // parse things out and build :)

    // Get all the prizes. These are in order for chrystals, green pendant first for pendants.
    let mut crystals: [String; 7] = [ String::new(), String::new(), String::new(), String::new(), String::new(), String::new(), String::new() ];
    let mut pendants: [String; 3] = [ String::new(), String::new(), String::new() ];

    let mut prize_map = HashMap::new();
    prize_map.insert( String::from("Eastern Palace"), String::from("Eastern Palace - Prize:1") );
    prize_map.insert( String::from("Desert Palace"), String::from("Desert Palace - Prize:1") );
    prize_map.insert( String::from("Tower Of Hera"), String::from("Tower of Hera - Prize:1") );
    prize_map.insert( String::from("Dark Palace"), String::from("Palace of Darkness - Prize:1") );
    prize_map.insert( String::from("Swamp Palace"), String::from("Swamp Palace - Prize:1") );
    prize_map.insert( String::from("Skull Woods"), String::from("Skull Woods - Prize:1") );
    prize_map.insert( String::from("Thieves Town"), String::from("Thieves' Town - Prize:1") );
    prize_map.insert( String::from("Ice Palace"), String::from("Ice Palace - Prize:1") );
    prize_map.insert( String::from("Misery Mire"), String::from("Misery Mire - Prize:1") );
    prize_map.insert( String::from("Turtle Rock"), String::from("Turtle Rock - Prize:1") );

    for (key, value) in prize_map.iter() {
        insert_crystal_if_exists(
            String::from(key),
            String::from ( json[ key ][ value ].as_str().unwrap() ),
            &mut crystals
        );

        insert_pendant_if_exists(
            String::from(key),
            String::from ( json[ key ][ value ].as_str().unwrap() ),
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
        for (key, value) in json[location].as_object().unwrap().iter() {
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
            } else if String::from( value.as_str().unwrap() ).starts_with("Compass") {
                compasses.push(
                    Compass {
                        compass_name: String::from( value.as_str().unwrap() ),
                        compass_location: String::from(key),
                    }
                );
            } else if String::from( value.as_str().unwrap() ).starts_with("Map") {
                maps.push(
                    DungeonMap {
                        map_name: String::from( value.as_str().unwrap() ),
                        map_location: String::from(key),
                    }
                );
            } else if String::from( value.as_str().unwrap() ).starts_with("BigKey") {
                big_keys.push(
                    BigKey {
                        key_name: String::from( value.as_str().unwrap() ),
                        key_location: String::from(key),
                    }
                );
            } else if String::from( value.as_str().unwrap() ).starts_with("Key") {
                small_keys.push(
                    SmallKey {
                        key_name: String::from( value.as_str().unwrap() ),
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
                        item_name: String::from( value.as_str().unwrap() ),
                        item_location: String::from(key),
                    }
                );
            }
        }
    }

    // assemble final product
    let race_log = RaceLog{
        enemizer: Enemizer {
            boss_shuffle:  String::from( json["meta"]["enemizer.boss_shuffle"].as_str().unwrap() ),
            enemy_shuffle: String::from( json["meta"]["enemizer.enemy_shuffle"].as_str().unwrap() ),
            enemy_damage:  String::from( json["meta"]["enemizer.enemy_damage"].as_str().unwrap() ),
            enemy_health:  String::from( json["meta"]["enemizer.enemy_health"].as_str().unwrap() ),
        },
        
        mode: String::from( json["meta"]["mode"].as_str().unwrap() ),
        goal: String::from( json["meta"]["goal"].as_str().unwrap() ),
        entry_crystals_ganon: String::from( json["meta"]["entry_crystals_ganon"].as_str().unwrap() ),
        entry_crystals_tower: String::from( json["meta"]["entry_crystals_tower"].as_str().unwrap() ),
        item_placement: String::from( json["meta"]["item_placement"].as_str().unwrap() ),
        item_pool: String::from( json["meta"]["item_pool"].as_str().unwrap() ),
        item_functionality: String::from( json["meta"]["item_functionality"].as_str().unwrap() ),
        dungeon_items: String::from( json["meta"]["dungeon_items"].as_str().unwrap() ),
        logic: String::from( json["meta"]["logic"].as_str().unwrap() ),
        accessibility: String::from( json["meta"]["accessibility"].as_str().unwrap() ),
        weapons: String::from( json["meta"]["weapons"].as_str().unwrap() ),
        hints: String::from( json["meta"]["hints"].as_str().unwrap() ),
        spoilers: String::from( json["meta"]["spoilers"].as_str().unwrap() ),
        build: String::from( json["meta"]["build"].as_str().unwrap() ),

        waterfall_fairy: String::from( json["Special"]["Waterfall Bottle:1"].as_str().unwrap() ),
        pyramid_fairy: String::from( json["Special"]["Pyramid Bottle:1"].as_str().unwrap() ),
        
        turtle_rock_medallion: String::from( json["Special"]["Turtle Rock Medallion:1"].as_str().unwrap() ),
        misery_mire_medallion: String::from( json["Special"]["Misery Mire Medallion:1"].as_str().unwrap() ),

        bosses: Bosses {
            eastern_palace: String::from( json["Bosses"]["Eastern Palace"].as_str().unwrap() ),
            desert_palace: String::from( json["Bosses"]["Desert Palace"].as_str().unwrap() ),
            tower_of_hera: String::from( json["Bosses"]["Tower Of Hera"].as_str().unwrap() ),
            hyrule_castle: String::from( json["Bosses"]["Hyrule Castle"].as_str().unwrap() ),
            palace_of_darkness: String::from( json["Bosses"]["Palace Of Darkness"].as_str().unwrap() ),
            swamp_palace: String::from( json["Bosses"]["Swamp Palace"].as_str().unwrap() ),
            skull_woods: String::from( json["Bosses"]["Skull Woods"].as_str().unwrap() ),
            thieves_town: String::from( json["Bosses"]["Thieves Town"].as_str().unwrap() ),
            ice_palace: String::from( json["Bosses"]["Ice Palace"].as_str().unwrap() ),
            misery_mire: String::from( json["Bosses"]["Misery Mire"].as_str().unwrap() ),
            turtle_rock: String::from( json["Bosses"]["Turtle Rock"].as_str().unwrap() ),
            ganons_tower_basement: String::from( json["Bosses"]["Ganons Tower Basement"].as_str().unwrap() ),
            ganons_tower_middle: String::from( json["Bosses"]["Ganons Tower Middle"].as_str().unwrap() ),
            ganons_tower_top: String::from( json["Bosses"]["Ganons Tower Top"].as_str().unwrap() ),
            ganons_tower: String::from( json["Bosses"]["Ganons Tower"].as_str().unwrap() ),
            ganon: String::from( json["Bosses"]["Ganon"].as_str().unwrap() ),
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
