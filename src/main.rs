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

    let parsed_weapon_rolls: Vec<WeaponRoll> = Vec::from([]);
    let weapon_roll_hash_table: HashMap<String, i128> = HashMap::new();
    for pair in voltron {
        if pair.as_rule() == Rule::weapon_rolls {
            let mut weapon_roll = WeaponRoll::new();

            for inner_pair in pair.into_inner() {
                if inner_pair.as_rule() == Rule::weapon_notes {
                    let weapon_notes = inner_pair.as_str();
                    let notes_string = weapon_notes.split("tags:").nth(0).unwrap_or("");
                    let tags_string = weapon_notes.split("tags:").nth(1).unwrap_or("");
                    weapon_roll.add_tags_from_text(tags_string);
                    weapon_roll.note = String::from(notes_string);
                }
                if inner_pair.as_rule() == Rule::rolls {
                    for roll in inner_pair.into_inner() {
                        for id in roll.into_inner() {
                            if id.as_rule() == Rule::id {
                                print!("Found id: {}", id.as_str())
                            }
                            if id.as_rule() == Rule::perks {
                                print!("Found perks: {}", id.as_str())
                            }
                        }
                        print!("\n")
                    }
                }
            }
            // print!("{}", weapon_roll.note);
            // print!("\n");
            // print!("{}", weapon_roll.tags.join(","));
            // print!("\n");
        }
    }

    // if let Err(e) = dim_wishlist_cleanup::run(config) {
    //     println!("Application error: {e}");
    //     exit(1);
    // }
}
