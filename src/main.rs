extern crate pest;
#[macro_use]
extern crate pest_derive;

mod structs;

use pest::{
    Parser,
};
use structs::wishlist::Wishlist;
use std::cmp::Ordering;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::exit;

#[derive(Parser)]
#[grammar = "voltron.pest"]
struct VoltronParser;

fn sort_by_god_rolls(current: &Wishlist, next: &Wishlist) -> std::cmp::Ordering {
    if next.is_god_roll() ^ current.is_god_roll() {
        if next.is_god_roll() {
            return Ordering::Greater;
        }
        return Ordering::Less;
    }
    next.tags.len().cmp(&current.tags.len())
}

fn handle_write_to_file(mut file: &File, bytes: &[u8]) {
    let bytes_written_success = file.write(bytes).unwrap_or_else(|err| {
        print!(
            "Problem writing the following bytes: {:?}, error: {}",
            bytes, err
        );
        0
    }) as u32;
    if bytes_written_success == 0 {
        exit(1);
    }
}

fn main() {
    let mkb_wishlist: File = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open("mkb_wishlist.txt")
        .unwrap();

    let file_contents = fs::read_to_string("voltron.txt").unwrap_or_else(|err| {
        println!("Problem reading file to memory: {err}");
        exit(1);
    });

    let weapon_rolls = VoltronParser::parse(Rule::voltron, &file_contents).unwrap_or_else(|err| {
        println!("Problem parsing voltron file: {err}");
        exit(1);
    });

    let mut parsed_wishlists: Vec<Wishlist> = Vec::from([]);

    for weapon_roll in weapon_rolls {
        let wishlist = Wishlist::new(weapon_roll.into_inner());
        parsed_wishlists.push(wishlist);
    }

    handle_write_to_file(&mkb_wishlist, b"title:Cobes-3's Reduced MNK Wishlist.\n");
    handle_write_to_file(&mkb_wishlist, b"description:This is a reduced wishlist that removes controller specific rolls from 48klocs voltron file. It also sorts rolls with tags to the top.\n\n");
    
    parsed_wishlists.sort_by(sort_by_god_rolls);

    for wishlist in parsed_wishlists {
        if !wishlist.is_empty() {
            handle_write_to_file(&mkb_wishlist, format!("\n{}", wishlist.note).as_bytes());
            if !wishlist.tags.is_empty() {
                handle_write_to_file(
                    &mkb_wishlist,
                    format!(" tags:{}\n", wishlist.tags.join(", ")).as_bytes(),
                );
            } else {
                handle_write_to_file(&mkb_wishlist, b"\n");
            }

            for weapon_roll in wishlist.weapon_rolls {
                handle_write_to_file(
                    &mkb_wishlist,
                    format!(
                        "dimwishlist:item={}&perks={}\n",
                        weapon_roll.item_id,
                        weapon_roll.perks.join(",")
                    )
                    .as_bytes(),
                );
            }
        }
    }
}
