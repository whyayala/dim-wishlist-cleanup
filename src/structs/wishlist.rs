use crate::services::deserialize_service::explode;

use super::weapon_roll::WeaponRoll;
extern crate pest;

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
            .replace([')', '|', '+', '\n', ' '], "")
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
}
