extern crate pest;
#[macro_use]
extern crate pest_derive;

mod structs;

use pest::Parser;
use std::collections::HashMap;
use std::env;
use std::process::exit;
use std::fs;

use crate::structs::weapon_roll::*;

use dim_wishlist_cleanup::Config;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct VoltronParser;    

fn split_weapon_notes(notes: &str) -> (&str, &str) {
    let notes_string = notes.split("tags:").nth(0).unwrap_or("");
    let tags_string = notes.split("tags:").nth(1).unwrap_or("");
    (notes_string, tags_string)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    let file_contents = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
        println!("Problem reading file to memory: {err}");
        exit(1);
    }).to_owned();

    let voltron = VoltronParser::parse(Rule::voltron, &file_contents).unwrap_or_else(|err| {
        println!("Problem parsing voltron file: {err}");
        exit(1);
    });

    let mut parsed_weapon_rolls: Vec<WeaponRoll> = Vec::from([]);
    let mut weapon_roll_hash_table: HashMap<String, i128> = HashMap::new();
    for pair in voltron {
        if pair.as_rule() == Rule::weapon_rolls {
            for inner_pair in pair.into_inner() {
                let mut weapon_roll = WeaponRoll::new();

                if inner_pair.as_rule() == Rule::weapon_notes {
                    let (notes_string, tags_string) = split_weapon_notes(inner_pair.as_str());
                    weapon_roll.add_tags_from_text(tags_string);
                    weapon_roll.note = String::from(notes_string);
                }
                if inner_pair.as_rule() == Rule::rolls {
                    for roll in inner_pair.into_inner() {
                        for roll_value in roll.into_inner() {
                            if roll_value.as_rule() == Rule::id {
                                weapon_roll.item_id = roll_value.as_str().to_string();
                            }
                            if roll_value.as_rule() == Rule::perks {
                                weapon_roll.add_perks_from_text(roll_value.into_inner().as_str())
                            }
                        }
                        let weapon_roll_for_vector = weapon_roll.to_owned();
                        parsed_weapon_rolls.push(weapon_roll_for_vector);
                            
                        weapon_roll_hash_table.insert(
                            weapon_roll.get_weapon_roll_id(),
                            0
                        );

                        weapon_roll.perks = Vec::from([]);
                        weapon_roll.tags = Vec::from([]);
                    }
                }
            }
        }
    }

    print!("Unique rolls: {}", weapon_roll_hash_table.len());
    print!("Total rolls: {}", parsed_weapon_rolls.len());

    // if let Err(e) = dim_wishlist_cleanup::run(config) {
    //     println!("Application error: {e}");
    //     exit(1);
    // }
}
