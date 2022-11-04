fn explode(string: &str, delimiter: &str) -> Vec<String> {
    let vec_of_strings: Vec<String> = string.split(delimiter)
        .map(|value| -> String {
            value.trim().to_lowercase().to_string()
        }).collect();
    vec_of_strings
}

#[derive(Clone)]
pub struct Wishlist {
    pub title: String,
    pub subtitle: String,
    pub note: String,
    pub tags: Vec<String>,
    pub rolls: Vec<WishlistItem>
}

#[derive(Clone)]
pub struct WishlistItem {
    pub item_id: String,
    pub perks: Vec<String>,
    pub note: String,
    pub tags: Vec<String>,
}

impl Wishlist {
    pub fn add_tags(&mut self, text: &str) -> () {
        self.tags = explode(text, ",");
    }
    pub fn new() -> Wishlist {
        Wishlist { 
            title: String::from(""),
            subtitle: String::from(""),
            note: String::from(""),
            tags: Vec::from([]),
            rolls: Vec::from([])
        }
    }
}

impl WishlistItem {
    pub fn new(text: &str) -> Result<WishlistItem, &'static str> {  
        let (_, item_and_perks) = text.split_once(":").unwrap_or(("", ""));
        let (item, perks_and_notes) = item_and_perks.split_once("&").unwrap_or(("", ""));
        let (_, perks) = perks_and_notes.split_once("=").unwrap_or(("", ""));
        let (parsed_perks, notes_and_tags) = 
            if perks.contains("#") {
                let (unparsed_perks, notes) = perks.split_once("#").unwrap_or(("", ""));
                let parsed_perks = explode(unparsed_perks, ",");
                (parsed_perks, notes.to_string())
            }
            else {
                let parsed_perks = explode(perks, ",");
                (parsed_perks, String::from(""))
            };
        let (notes, tags) = 
            if notes_and_tags.contains("tags:") {
                let (parsed_notes, unparsed_tags) = notes_and_tags.split_once("tags:").unwrap_or(("", ""));
                let parsed_tags = explode(unparsed_tags, ",");
                (parsed_notes.to_string(), parsed_tags)
            }
            else {
                (notes_and_tags, Vec::from([]))
            };
        Ok(WishlistItem { 
            item_id: String::from(item.split("=").last().unwrap_or("")),
            perks: parsed_perks,
            note: notes,
            tags: tags
        })
    }
}
