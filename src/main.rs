use std::env;
use std::process::exit;

use dim_wishlist_cleanup::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        exit(1);
    });

    if let Err(e) = dim_wishlist_cleanup::run(config) {
        println!("Application error: {e}");
        exit(1);
    }
}
