extern crate pest;
#[macro_use]
extern crate pest_derive;

mod structs;

use pest::{Parser, iterators::Pairs};
use std::collections::HashMap;
// use std::env;
use std::process::exit;
use std::fs;

use crate::structs::weapon_roll::{*, self};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct VoltronParser;    

fn split_weapon_notes(notes: &str) -> (&str, &str) {
    let notes_string = notes.split("tags:").nth(0).unwrap_or("");
    let tags_string = notes.split("tags:").nth(1).unwrap_or("");
    (notes_string, tags_string)
}

fn is_controller_specific(tags_string: &str) -> bool {
    tags_string.to_lowercase().contains("controller") && !tags_string.to_lowercase().contains("mkb")
}

fn get_weapon_rolls(weapon_note_and_rolls: Pairs<Rule>) -> Vec<WeaponRoll> {
    let mut notes_string: &str = "";
    let mut tags_string: &str = "";
    weapon_note_and_rolls.fold(
        Vec::from([]),
        |accumulator: Vec<WeaponRoll>, element| {
            if element.as_rule() == Rule::weapon_notes {
                (notes_string, tags_string) = split_weapon_notes(element.as_str());
                accumulator
            }
            else if element.as_rule() == Rule::roll && !is_controller_specific(tags_string) {
                let roll_id_and_perks = element.into_inner();
                let new_roll = roll_id_and_perks.fold(
                    WeaponRoll::new(),
                    | accumulator, roll_value | {

                    if roll_value.as_rule() == Rule::id {
                        let new_accumulator = WeaponRoll {
                            item_id: roll_value.as_str().to_string(),
                            ..accumulator
                        };
                        new_accumulator
                    }
                    else if roll_value.as_rule() == Rule::perks {
                        let mut new_accumulator = WeaponRoll {
                            ..accumulator
                        };
                        new_accumulator.note = notes_string.to_string();
                        new_accumulator.add_tags_from_text(tags_string);
                        new_accumulator.add_perks_from_text(roll_value.into_inner().as_str());
                        new_accumulator
                    }
                    else if roll_value.as_rule() == Rule::notes {
                        let mut new_accumulator = WeaponRoll {
                            ..accumulator
                        };
                        (notes_string, tags_string) = split_weapon_notes(roll_value.as_str());
                        new_accumulator.note = new_accumulator.note + notes_string;
                        new_accumulator.add_tags_from_text(tags_string);
                        new_accumulator
                    }
                    else {
                        accumulator
                    }
                });

                [accumulator, vec!(new_roll)].concat()
            }
            else {
                accumulator
            }
        }
    )
}

fn main() {
    let file_contents = fs::read_to_string("voltron.txt").unwrap_or_else(|err| {
        println!("Problem reading file to memory: {err}");
        exit(1);
    }).to_owned();

    let weapon_rolls = VoltronParser::parse(Rule::voltron, &file_contents).unwrap_or_else(|err| {
        println!("Problem parsing voltron file: {err}");
        exit(1);
    });

    let mut parsed_weapon_rolls: Vec<WeaponRoll> = Vec::from([]);
    let mut weapon_roll_hash_table = HashMap::new();

    for weapon_roll in weapon_rolls {
        let weapon_note_and_rolls = weapon_roll.into_inner();

        let mut values = get_weapon_rolls(weapon_note_and_rolls);
        for value in &values {
            weapon_roll_hash_table.insert(value.get_weapon_roll_id(), 0);
            // TODO: Add println to create new voltron.txt
        }
        parsed_weapon_rolls.append(&mut values)
    }

    print!("title:This is a reduced wishlist pulled from 48klocs project that removes rolls tagged with controller and not mkb.\n");
    print!("description:This is still a work in progress.\n\n");
    for parsed_weapon_roll in parsed_weapon_rolls {
        // print!("//{}", parsed_weapon_roll.note);
        // print!("tags:{}\n", parsed_weapon_roll.tags.join(", "));
        print!("dimwishlist:item={}", parsed_weapon_roll.item_id);
        print!("&perks={}\n", parsed_weapon_roll.perks.join(","));
    }
    
    // if let Err(e) = dim_wishlist_cleanup::run(config) {
    //     println!("Application error: {e}");
    //     exit(1);
    // }
}
