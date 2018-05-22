extern crate minigrep;

use std::env;
use std::process;
use minigrep::*;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = query::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
