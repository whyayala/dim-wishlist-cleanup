extern crate pest;
#[macro_use]
extern crate pest_derive;

mod structs;

use pest::{Parser, iterators::{Pairs, Pair}};
use structs::wishlist::Wishlist;
// use std::env;
use std::process::exit;
use std::cmp::Ordering;
use std::fs;

use crate::structs::{weapon_roll::*};

#[derive(Parser)]
#[grammar = "voltron.pest"]
struct VoltronParser;    

fn split_weapon_notes(notes: &str) -> (&str, &str) {
    let (note, tags) = if notes.contains("tags:") {
        notes.rsplit_once("tags:").unwrap_or(("", ""))
    } else {
        notes.rsplit_once("Tags: ").unwrap_or(("", ""))
    };
    
    if tags.is_empty() {
        (notes, tags_from_notes(&notes))
    } else {
        (note, tags)
    }
}

fn is_controller_specific(tags_string: &str) -> bool {
    tags_string.to_lowercase().contains("controller") && !tags_string.to_lowercase().contains("mkb") && !tags_string.to_lowercase().contains("m+kb")
}

fn tags_from_notes(notes_string: &str) -> &str {
    if notes_string.contains("(PvP backup roll)") {
        "pvp"
    } else if notes_string.contains("(PvE backup roll)") || notes_string.contains("(PvE backupe roll)") {
        "pve"
    } else if notes_string.contains("(PvE first choice roll)") {
        "pve,pve-god"
    } else if notes_string.contains("(PvP first choice roll)") {
        "pvp,pvp-god"
    } else {
        ""
    }
}

fn is_not_great(notes_string: &str) -> bool {
    notes_string.contains("(not great PvP)") ||
    notes_string.contains("(not great PvE)")
}

fn is_desirable_roll(tags_string: &str, notes_string: &str, pair: &Pair<Rule>) -> bool {
    pair.as_rule() == Rule::roll && !is_controller_specific(tags_string) && !is_not_great(notes_string)
}

fn sort_by_god_rolls(current: &Wishlist, next: &Wishlist) -> std::cmp::Ordering {
    if next.is_god_roll() ^ current.is_god_roll() {
        if next.is_god_roll() {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
    else if next.is_god_roll() && current.is_god_roll() {
        return Ordering::Equal;
    }
    next.tags.len().cmp(&current.tags.len())
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

    for weapon_roll in weapon_rolls {
        let wishlist = get_weapon_rolls(weapon_roll.into_inner());
        parsed_wishlists.push(wishlist)
    }

    print!("title:Cobes-3's Reduced MNK Wishlist.\n");
    print!("description:This is a reduced wishlist that removes controller specific rolls from 48klocs voltron file. It also sorts rolls with tags to the top.\n\n");

    parsed_wishlists.sort_by(
        |current, next| sort_by_god_rolls(current, next)
    );

    for wishlist in parsed_wishlists {
        if !wishlist.is_empty() {
            print!("\n{}", wishlist.note);
            if wishlist.tags.len() > 0 {
                print!(" tags:{}\n", wishlist.tags.join(", "));
            } else {print!{"\n"}}
            for weapon_roll in wishlist.weapon_rolls {
                print!("dimwishlist:item={}", weapon_roll.item_id);
                print!("&perks={}\n", weapon_roll.perks.join(","));
            }
        }
    }
    
    // if let Err(e) = dim_wishlist_cleanup::run(config) {
    //     println!("Application error: {e}");
    //     exit(1);
    // }
}
