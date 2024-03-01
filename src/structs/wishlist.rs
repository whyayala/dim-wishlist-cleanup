use super::weapon_roll::WeaponRoll;
use crate::Rule;
extern crate pest;

use pest::iterators::{Pair, Pairs};

fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string
        .split(delimiter)
        .map(|value| -> String { value.trim().to_string() })
        .collect();
    vec_of_strings
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

fn is_controller_specific(tags_string: &str) -> bool {
    tags_string.to_lowercase().contains("controller")
        && !tags_string.to_lowercase().contains("mkb")
        && !tags_string.to_lowercase().contains("m+kb")
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
    notes_string.contains("(not great PvP)") || notes_string.contains("(not great PvE)")
}

fn is_desirable_roll(tags_string: &str, notes_string: &str, pair: &Pair<Rule>) -> bool {
    pair.as_rule() == Rule::roll
        && !tags_string.is_empty()
        && !is_controller_specific(tags_string)
        && !is_not_great(notes_string)
        && !notes_string.to_lowercase().contains("pandapaxxy")
        && !notes_string.to_lowercase().contains("yeezygt")
}

#[derive(Clone)]
pub struct Wishlist {
    pub note: String,
    pub tags: Vec<String>,
    pub weapon_rolls: Vec<WeaponRoll>,
}

impl Wishlist {
    fn get_possible_tags() -> [String; 13] {
        [
            String::from("controller"),
            String::from("mkb"),
            String::from("pve"),
            String::from("pve-endgame"),
            String::from("pve-champion"),
            String::from("pve-minor"),
            String::from("pve-major"),
            String::from("pve-boss"),
            String::from("pve-god"),
            String::from("pvp"),
            String::from("pvp-duel"),
            String::from("pvp-chain"),
            String::from("pvp-god"),
        ]
    }

    pub fn add_tags_from_text(&mut self, text: &str) {
        let tag_array = Self::get_possible_tags();
        let cleaned_text = text
            .to_lowercase()
            .replace([')', '|', '+', '\n'], "")
            .replace("god-pve", "pve-god")
            .replace("god-pvp", "pvp-god")
            .replace("pve=endgame", "pve-endgame")
            .replace("pve-minorspec", "pve-minor")
            .replace("pve-majorspec", "pve-major")
            .replace("pve-bossspec", "pve-boss")
            .replace("pve-pve-champions", "pve-champion")
            .replace("pvp-duelling", "pvp-duel")
            .replace("pvp-killchain", "pvp-chain")
            .replace("pvp-chaining", "pvp-chain");
        let exploded_text = explode(&cleaned_text, ",");
        for item in exploded_text {
            if tag_array.contains(&item) && !self.tags.contains(&item) {
                self.tags.push(item)
            }
        }
    }

    pub fn add_notes_from_text(&mut self, text: &str) {
        self.note = text.replace(['|', '(', ')'], "").trim().to_string();
    }

    pub fn is_empty(&self) -> bool {
        self.weapon_rolls.is_empty()
    }

    pub fn is_god_roll(&self) -> bool {
        self.tags.contains(&String::from("pve-god")) || self.tags.contains(&String::from("pvp-god"))
    }

    pub fn new(weapon_note_and_rolls: Pairs<Rule>) -> Wishlist {
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
}
