use std::error::Error;
use std::fs;
use std::collections::HashMap;

mod structs;
use crate::structs::wishlist::*;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided. (ex. cargo run -- keyword filename.txt)");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let wishlist_file: String = fs::read_to_string(config.file_path)?;
    let mut aggregate_wishlists: HashMap<String, Wishlist> = HashMap::new();
    for wishlist in search(&config.query, &wishlist_file)? {
        // if wishlist.title.contains("BxR-55 Battler (PvP first-choice roll)") {
        //     aggregate_wishlist.title = String::from("BxR-55 Battler (PvP first-choice roll)");
        //     aggregate_wishlist.rolls = [&aggregate_wishlist.rolls[..], &wishlist.rolls[..]].concat();
        //     println!("Wishlist title {}", wishlist.title);
        //     println!("    subtitle {}", wishlist.subtitle);
        //     println!("    note {}", wishlist.note);
        //     println!("    tags {}", wishlist.tags.join(","));
        //     println!("    roll count {}", wishlist.rolls.len());
        // }
        let borrowed_title = &wishlist.title;
        let borrowed_tags = &wishlist.tags.join(",");
        let aggregate_wishlist_hash = format!("{}{}", borrowed_title, borrowed_tags);
        if aggregate_wishlists.contains_key(&aggregate_wishlist_hash) {
            let old_wishlist = aggregate_wishlists.get(&aggregate_wishlist_hash).unwrap().to_owned();
            let old_rolls = &old_wishlist.rolls;
            let new_rolls = &wishlist.rolls;
            let new_wishlist = Wishlist {
                title: old_wishlist.title,
                subtitle: old_wishlist.subtitle,
                note: old_wishlist.note,
                tags: old_wishlist.tags,
                rolls: [&new_rolls[..], &old_rolls[..]].concat()
            };
            aggregate_wishlists.insert(aggregate_wishlist_hash, new_wishlist);
        }
        else {
            aggregate_wishlists.insert(aggregate_wishlist_hash, wishlist);
        }
    }
    println!("Total aggregate wishlists: {}", aggregate_wishlists.len());
    // println!("    roll count: {}", aggregate_wishlist.rolls.len());
    let mut perk_combos:HashMap<String, i32> = HashMap::new();
    // for roll in aggregate_wishlist.rolls {
    //     let perk_combo: String = roll.perks.join(", ");
    //     if perk_combos.contains_key(&perk_combo) {
    //         perk_combos.insert(perk_combo.to_string(), perk_combos[&perk_combo] + 1)
    //     }
    //     else {
    //         perk_combos.insert(perk_combo, 1)
    //     };
    //     // println!("{}", roll.text);
    //     println!("perks: {} tags: {}", roll.perks.join(", "), roll.tags.join(", "));
    // }

    // for (perk_combo, weight) in perk_combos {
    //     println!("Perk combo: {}, weight: {}", perk_combo, weight)
    // }
    // println!("Final aggregate wishlist:");
    // println!("Final aggregate wishlist:");
    // println!("Final aggregate wishlist:");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<Wishlist>, Box<dyn Error>> {
    let mut wishlists: Vec<Wishlist> = Vec::new();
    let mut empty_wishlist: Wishlist = Wishlist::new();

    for line in contents.lines() {
        if line.starts_with("//") {
            let cleaned_line = line.strip_prefix("//").unwrap_or("");
            if cleaned_line.starts_with("notes:") {
                if cleaned_line.contains("tags:") {
                    let (notes, tags) = cleaned_line.split_once("tags:").unwrap_or(("", ""));
                    empty_wishlist.note.push_str(notes);
                    empty_wishlist.add_tags(tags);
                }
                empty_wishlist.note.push_str(cleaned_line);
            }
            else {
                if empty_wishlist.title.is_empty() {
                    empty_wishlist.title.push_str(cleaned_line);
                }
                else {
                    empty_wishlist.subtitle.push_str(cleaned_line);                    
                }
            }
        }
        else if line.starts_with("dimwishlist") {
            let mut wishlist_item = WishlistItem::new(line).unwrap();
            let mut wishlist_tags = empty_wishlist.tags.to_vec();
            wishlist_item.tags.append(&mut wishlist_tags);
            empty_wishlist.rolls.push(wishlist_item);
        }
        else if line.eq("") {
            if empty_wishlist.rolls.is_empty() {
                empty_wishlist = Wishlist::new()
            }
            else {
                wishlists.push(empty_wishlist);
                empty_wishlist = Wishlist::new();
            }
        }
        // if line.contains(query)  {
        //     items.push(WishlistItem::new(line)?)
        // }
    }
    println!("Total valid wishlists: {}", wishlists.len());
    Ok(wishlists)
}