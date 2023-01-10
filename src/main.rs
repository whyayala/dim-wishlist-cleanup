extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::env;
use std::process::exit;

use dim_wishlist_cleanup::Config;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct WishlistParser;    

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    let wishlist_items = WishlistParser::parse(Rule::rolls, &config.file_path).unwrap();
    
    for line in wishlist_items.into_iter() {
        print!("{}", line)
    }


    if let Err(e) = dim_wishlist_cleanup::run(config) {
        println!("Application error: {e}");
        exit(1);
    }
}
