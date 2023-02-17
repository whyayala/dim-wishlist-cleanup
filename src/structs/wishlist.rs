use super::weapon_roll::WeaponRoll;

#[derive(Clone)]
pub struct Wishlist {
    pub note: String,
    pub weapon_rolls: Vec<WeaponRoll>,
}

impl Wishlist {
    // fn get_possible_tags() -> [String; 14] { [
    //     String::from("pvp"),
    //     String::from("pve"),
    //     String::from("mkb"),
    //     String::from("controller"),
    //     String::from("pvp-duelling"),
    //     String::from("pvp-killchain"),
    //     String::from("pve-endgame"),
    //     String::from("pve-champion"),
    //     String::from("pvp-god"),
    //     String::from("pve-minorspec"), 
    //     String::from("pve-majorspec"),
    //     String::from("pve-bossspec"),
    //     String::from("pve-boss"),
    //     String::from("pve-god")
    // ]}

    // pub fn add_tags_from_text(&mut self, text: &str) -> () {
    //     let tag_array = Self::get_possible_tags();
    //     let cleaned_text = text.replace(&[')', '|'], "");
    //     let exploded_text = explode(&cleaned_text, ",");
    //     for item in exploded_text {
    //         if tag_array.contains(&item) && !self.tags.contains(&item){
    //             self.tags.push(item)
    //         }
    //     }
    // }

    // pub fn add_perks_from_text(&mut self, text: &str) -> () {
    //     let exploded_text = explode(text, ",");
    //     for item in exploded_text {
    //         self.perks.push(item);
    //     }
    // }

    // pub fn get_weapon_roll_id(&self) -> String {
    //     let mut concatenated_string = String::from("");
    //     let cloned_perks = &mut self.perks.to_owned();
    //     cloned_perks.sort();
    //     concatenated_string.push_str(&self.item_id);
    //     concatenated_string.push_str(cloned_perks.join("").as_str());
    //     concatenated_string
    // }

    pub fn new() -> Wishlist {
        Wishlist { 
            // item_id: String::from(""),
            note: String::from(""),
            // tags: Vec::from([]),
            weapon_rolls: Vec::from([]),
        }
    }
}
