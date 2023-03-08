extern crate pest;
#[macro_use]
extern crate pest_derive;

mod structs;

use pest::{Parser, iterators::{Pairs, Pair}};
use structs::wishlist::Wishlist;
use std::collections::HashMap;
// use std::env;
use std::process::exit;
use std::fs;

use crate::structs::{weapon_roll::*};

#[derive(Parser)]
#[grammar = "voltron.pest"]
struct VoltronParser;    

fn split_weapon_notes(notes: &str) -> (&str, &str) {
    let (note, tags) = notes.rsplit_once("tags:").unwrap_or(("", ""));
    if tags.is_empty() {
        (notes, "")
    } else {
        (note, tags)
    }
}

fn is_controller_specific(tags_string: &str) -> bool {
    tags_string.to_lowercase().contains("controller") && !tags_string.to_lowercase().contains("mkb") && !tags_string.to_lowercase().contains("m+kb")
}

fn is_first_choice(notes_string: &str) -> bool {
    !notes_string.contains("(PvP backup roll)") &&
    !notes_string.contains("(PvE backup roll)") &&
    !notes_string.contains("(PvE backupe roll)") &&
    !notes_string.contains("(not great PvP)") &&
    !notes_string.contains("(not great PvE)")
}

fn is_desirable_roll(tags_string: &str, notes_string: &str, pair: &Pair<Rule>) -> bool {
    pair.as_rule() == Rule::roll && !is_controller_specific(tags_string) && is_first_choice(notes_string)
}

fn get_weapon_rolls(weapon_note_and_rolls: Pairs<Rule>) -> Wishlist {
    let mut notes_string: &str = "";
    let mut tags_string: &str = "";
    weapon_note_and_rolls.fold(
        Wishlist::new(),
        |wishlist_accumulator: Wishlist, element| {
            if element.as_rule() == Rule::weapon_notes {
                (notes_string, tags_string) = split_weapon_notes(element.as_str());
                let mut new_wishlist_accumulator = Wishlist {
                    ..wishlist_accumulator
                };
                new_wishlist_accumulator.add_notes_from_text(notes_string);
                new_wishlist_accumulator.add_tags_from_text(tags_string);
                new_wishlist_accumulator
            }
            else if is_desirable_roll(tags_string, notes_string, &element) {
                let mut new_wishlist_accumulator = Wishlist {
                    ..wishlist_accumulator
                };
                let roll_id_and_perks = element.into_inner();
                let new_roll = roll_id_and_perks.fold(
                    WeaponRoll::new(),
                    | roll_accumulator, roll_value | {

                        if roll_value.as_rule() == Rule::id {
                            let new_roll_accumulator = WeaponRoll {
                                item_id: roll_value.as_str().to_string(),
                                ..roll_accumulator
                            };
                            new_roll_accumulator
                        }
                        else if roll_value.as_rule() == Rule::perks {
                            let mut new_roll_accumulator = WeaponRoll {
                                ..roll_accumulator
                            };
                            new_roll_accumulator.add_perks_from_text(roll_value.into_inner().as_str());
                            new_roll_accumulator
                        }
                        else if roll_value.as_rule() == Rule::notes {
                            (notes_string, tags_string) = split_weapon_notes(roll_value.as_str());
                            new_wishlist_accumulator.add_tags_from_text(tags_string);
                            roll_accumulator
                        }
                        else {
                            roll_accumulator
                        }
                    }
                );
                new_wishlist_accumulator.weapon_rolls.push(new_roll);
                new_wishlist_accumulator
            }
            else {
                wishlist_accumulator
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

    let mut parsed_wishlists: Vec<Wishlist> = Vec::from([]);
    let mut weapon_roll_hash_table = HashMap::new();

    for weapon_roll in weapon_rolls {
        let weapon_note_and_rolls = weapon_roll.into_inner();

        let wishlist = get_weapon_rolls(weapon_note_and_rolls);
        for weapon_roll in &wishlist.weapon_rolls {
            weapon_roll_hash_table.insert(weapon_roll.get_weapon_roll_id(), 0);
        }
        parsed_wishlists.push(wishlist)
    }

    print!("title:This is a reduced wishlist pulled from 48klocs project that removes rolls tagged with controller and not mkb.\n");
    print!("description:This is still a work in progress.\n\n");
    let mut untagged_wishlists: Vec<Wishlist> = Vec::from([]);
    let mut low_tagged_wishlists: Vec<Wishlist> = Vec::from([]);
    for parsed_wishlist in parsed_wishlists {
        if !parsed_wishlist.is_empty() {
            if parsed_wishlist.tags.is_empty() {
                untagged_wishlists.push(parsed_wishlist);
            }
            else if parsed_wishlist.tags.len() <= 3 {
                low_tagged_wishlists.push(parsed_wishlist);
            }
            else {
                print!("\n{}", parsed_wishlist.note);
                print!(" tags:{}\n", parsed_wishlist.tags.join(", "));
                for weapon_roll in parsed_wishlist.weapon_rolls {
                    print!("dimwishlist:item={}", weapon_roll.item_id);
                    print!("&perks={}\n", weapon_roll.perks.join(","));
                }
            }
        }
    }

    for parsed_wishlist in low_tagged_wishlists {
        print!("\n{}", parsed_wishlist.note);
        print!(" tags:{}\n", parsed_wishlist.tags.join(", "));
        for weapon_roll in parsed_wishlist.weapon_rolls {
            print!("dimwishlist:item={}", weapon_roll.item_id);
            print!("&perks={}\n", weapon_roll.perks.join(","));
        }
    }

    print!("\n// Untagged roll. Use with discretion.\n");
    for parsed_wishlist in untagged_wishlists {
        for weapon_roll in parsed_wishlist.weapon_rolls {
            print!("dimwishlist:item={}", weapon_roll.item_id);
            print!("&perks={}\n", weapon_roll.perks.join(","));
        }
    }
    
    // if let Err(e) = dim_wishlist_cleanup::run(config) {
    //     println!("Application error: {e}");
    //     exit(1);
    // }
}
