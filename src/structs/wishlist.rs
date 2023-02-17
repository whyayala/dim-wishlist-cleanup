use super::weapon_roll::WeaponRoll;

fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string.split(delimiter)
        .map(|value| -> String {
            value.trim().to_lowercase().to_string()
        }).collect();
    vec_of_strings
}

#[derive(Clone)]
pub struct Wishlist {
    pub note: String,
    pub tags: Vec<String>,
    pub weapon_rolls: Vec<WeaponRoll>,
}

impl Wishlist {
    fn get_possible_tags() -> [String; 14] { [
        String::from("pvp"),
        String::from("pve"),
        String::from("mkb"),
        String::from("controller"),
        String::from("pvp-duelling"),
        String::from("pvp-killchain"),
        String::from("pve-endgame"),
        String::from("pve-champion"),
        String::from("pvp-god"),
        String::from("pve-minorspec"), 
        String::from("pve-majorspec"),
        String::from("pve-bossspec"),
        String::from("pve-boss"),
        String::from("pve-god")
    ]}

    pub fn add_tags_from_text(&mut self, text: &str) -> () {
        let tag_array = Self::get_possible_tags();
        let cleaned_text = text.replace(&[')', '|'], "");
        let exploded_text = explode(&cleaned_text, ",");
        for item in exploded_text {
            if tag_array.contains(&item) && !self.tags.contains(&item){
                self.tags.push(item)
            }
        }
    }

    pub fn add_notes_from_text(&mut self, text: &str) -> () {
        self.note = text
            .strip_suffix("\n")
            .unwrap_or(text)
            .to_string();
    }

    pub fn is_empty(&self) -> bool {
        self.weapon_rolls.is_empty()
    }

    pub fn new() -> Wishlist {
        Wishlist { 
            // item_id: String::from(""),
            note: String::from(""),
            tags: Vec::from([]),
            weapon_rolls: Vec::from([]),
        }
    }
}
