extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::env;
use std::process::exit;
use std::fs;

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

    let file_contents = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
        println!("Problem reading file to memory: {err}");
        exit(1);
    }).to_owned();

    let wishlist= WishlistParser::parse(Rule::wishlist, &file_contents).unwrap_or_else(|err| {
        println!("Problem parsing wishlist file: {err}");
        exit(1);
    });
    
    print!("{}", wishlist);

    // for line in wishlist.into_iter() {
    //     print!("{}", line);
    //     print!("\n");
    // }


    // if let Err(e) = dim_wishlist_cleanup::run(config) {
    //     println!("Application error: {e}");
    //     exit(1);
    // }
}
