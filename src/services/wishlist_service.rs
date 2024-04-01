use pest::iterators::{Pair, Pairs};

use crate::{structs::{weapon_roll::WeaponRoll, wishlist::Wishlist}, Rule};

fn is_not_great(notes_string: &str) -> bool {
    notes_string.contains("(not great PvP)") || notes_string.contains("(not great PvE)") || notes_string.contains("(PvP backup roll)")
}

fn is_controller_specific(tags_string: &str) -> bool {
    tags_string.to_lowercase().contains("controller")
        && !tags_string.to_lowercase().contains("mkb")
        && !tags_string.to_lowercase().contains("m+kb")
}

fn is_desirable_roll(tags_string: &str, notes_string: &str, pair: &Pair<Rule>) -> bool {
    pair.as_rule() == Rule::roll
        && (tags_string.contains("god") || tags_string.contains("pve"))
        && !is_controller_specific(tags_string)
        && !is_not_great(notes_string)
        && !notes_string.to_lowercase().contains("yeezygt")
}

fn tags_from_notes(notes_string: &str) -> &str {
    if notes_string.contains("(PvE backup roll)") || notes_string.contains("(PvE backupe roll)") {
        "pve"
    } else if notes_string.contains("(PvE first choice roll)") || notes_string.contains("pve-god") || notes_string.contains("god-pve") {
        "pve,pve-god"
    } else if notes_string.contains("(PvP first choice roll)") || notes_string.contains("pvp-god") || notes_string.contains("god-pvp") {
        "pvp,pvp-god"
    } else {
        ""
    }
}

fn split_weapon_notes(notes: &str) -> (&str, &str) {
    let (note, tags) = if notes.contains("tags:") {
        notes.rsplit_once("tags:").unwrap_or(("", ""))
    } else {
        notes.rsplit_once("Tags: ").unwrap_or(("", ""))
    };

    if tags.is_empty() {
        (notes, tags_from_notes(notes))
    } else {
        (note, tags)
    }
}

pub fn get_wishlist(weapon_note_and_rolls: Pairs<Rule>) -> Wishlist {
    let mut notes_string: &str = "";
    let mut tags_string: &str = "";
    weapon_note_and_rolls.fold(
        Wishlist {
            note: String::from(""),
            tags: Vec::from([]),
            weapon_rolls: Vec::from([]),
        },
        |wishlist_accumulator: Wishlist, element| {
            if element.as_rule() == Rule::weapon_notes {
                (notes_string, tags_string) = split_weapon_notes(element.as_str());
                let mut new_wishlist_accumulator = Wishlist {
                    ..wishlist_accumulator
                };
                new_wishlist_accumulator.add_notes_from_text(notes_string);
                new_wishlist_accumulator.add_tags_from_text(tags_string);
                new_wishlist_accumulator
            } else if is_desirable_roll(tags_string, notes_string, &element) {
                let mut new_wishlist_accumulator = Wishlist {
                    ..wishlist_accumulator
                };
                let roll_id_and_perks = element.into_inner();
                let new_roll =
                    roll_id_and_perks.fold(WeaponRoll::new(), |roll_accumulator, roll_value| {
                        if roll_value.as_rule() == Rule::id {
                            let new_roll_accumulator = WeaponRoll {
                                item_id: roll_value.as_str().to_string(),
                                ..roll_accumulator
                            };
                            new_roll_accumulator
                        } else if roll_value.as_rule() == Rule::perks {
                            let mut new_roll_accumulator = WeaponRoll { ..roll_accumulator };
                            new_roll_accumulator
                                .add_perks_from_text(roll_value.into_inner().as_str());
                            new_roll_accumulator
                        } else if roll_value.as_rule() == Rule::notes {
                            (notes_string, tags_string) = split_weapon_notes(roll_value.as_str());
                            new_wishlist_accumulator.add_tags_from_text(tags_string);
                            roll_accumulator
                        } else {
                            roll_accumulator
                        }
                    });
                if !new_roll.clone().is_bad_perk_combo() {
                    new_wishlist_accumulator.weapon_rolls.push(new_roll);
                    new_wishlist_accumulator
                } else {
                    new_wishlist_accumulator
                }
            } else {
                wishlist_accumulator
            }
        },
    )
}
