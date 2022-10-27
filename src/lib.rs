use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided. (ex. cargo run -- keyword filename.txt)");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}


pub struct WishlistItem {
    pub item_id: String,
    pub perks: Vec<String>,
    pub text: String
}
pub struct Wishlist {
    pub note: String,
    pub tags: Vec<String>,
    pub rolls: Vec<WishlistItem>
}

impl WishlistItem {
    pub fn new(text: &str) -> Result<WishlistItem, &'static str> {        
        Ok(WishlistItem { text: text.to_string() })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let wishlist = fs::read_to_string(config.file_path)?;

    for item in search(&config.query, &wishlist)? {
        println!("{0}", item.text);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<WishlistItem>, Box<dyn Error>> {
    let mut items = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            items.push(WishlistItem::new(line)?)
        }
    }
    Ok(items)
}