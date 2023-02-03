fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string.split(delimiter)
        .map(|value| -> String {
            value.trim().to_lowercase().to_string()
        }).collect();
    vec_of_strings
}

#[derive(Clone)]
pub struct WeaponRoll {
    pub item_id: String,
    pub note: String,
    pub tags: Vec<String>,
    pub perks: Vec<String>,
    pub weapon_roll_id: String
}

impl WeaponRoll {
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
        let exploded_text = explode(text, ",");
        for item in exploded_text {
            if tag_array.contains(&item) && !self.tags.contains(&item){
                self.tags.push(item)
            }
        }
    }
    pub fn new() -> WeaponRoll {
        WeaponRoll { 
            item_id: String::from(""),
            note: String::from(""),
            tags: Vec::from([]),
            perks: Vec::from([]),
            weapon_roll_id: String::from("")
        }
    }
}
