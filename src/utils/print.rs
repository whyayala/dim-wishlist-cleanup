use crate::structs::wishlist::Wishlist;

pub fn print_wishlist_file(wishlists: Vec<Wishlist>) -> () {
    for wishlist in wishlists {
        println!("//Title: {}", wishlist.title);
        if !wishlist.subtitle.is_empty() {
            // println!("//Subtitle: {}", wishlist.subtitle);
        }
        if !wishlist.note.is_empty() || !wishlist.tags.is_empty() {
            if wishlist.note.is_empty() {
                // println!("//notes: No notes from wishlist |tags: {}", wishlist.tags.join(", "))
            }
            else if wishlist.tags.is_empty() {
                // println!("//{}", wishlist.note)
            }
            else {
                // println!("//{} |tags: {}", wishlist.note, wishlist.tags.join(", "))
            }
        }
        for roll in wishlist.rolls {
            // println!("dimwishlist:item={}&perks={}", roll.item_id, roll.perks.join(","));
        }
        // println!("");
        // println!("");
    }
}