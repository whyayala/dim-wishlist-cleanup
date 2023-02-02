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
struct VoltronParser;    

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

    let voltron = VoltronParser::parse(Rule::voltron, &file_contents).unwrap_or_else(|err| {
        println!("Problem parsing voltron file: {err}");
        exit(1);
    });
    
    // for line in wishlist.into_iter() {
    //     let cloned_line = line.clone();
    //     if cloned_line.as_rule() == Rule::wishlist_title || cloned_line.as_rule() == Rule::wishlist_description {
            
    //     }
    //     print!("{}", line.into_inner().as_str());
    // }

    let wishlist_line = voltron.into_iter().last().unwrap();
    
    for line in wishlist_line.into_inner() {

        print!("{}", line.into_inner());
    }

    // if let Err(e) = dim_wishlist_cleanup::run(config) {
    //     println!("Application error: {e}");
    //     exit(1);
    // }
}
