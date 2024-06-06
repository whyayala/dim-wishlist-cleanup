use pest::iterators::{Pair, Pairs};

use crate::{structs::{weapon_roll::WeaponRoll, wishlist::Wishlist}, Rule};

fn is_good_for_mnk(tags_string: &str) -> bool {
    !tags_string.to_lowercase().contains("controller") 
        || tags_string.to_lowercase().contains("mkb") 
        || tags_string.to_lowercase().contains("m+kb")
}

fn is_desirable_roll(tags_string: &str, pair: &Pair<Rule>) -> bool {
    pair.as_rule() == Rule::roll
        && (tags_string.to_lowercase().contains("god") || tags_string.to_lowercase().contains("endgame"))
        && is_good_for_mnk(tags_string)
}

fn split_weapon_notes(notes: &str) -> (&str, &str) {
    let (note, tags) = if notes.contains("tags:") {
        notes.rsplit_once("tags:").unwrap_or(("", ""))
    } else {
        notes.rsplit_once("Tags: ").unwrap_or(("", ""))
    };
    (note, tags)
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
            } else if is_desirable_roll(tags_string, &element) {
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
