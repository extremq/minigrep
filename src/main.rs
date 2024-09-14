use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err| {
        println!("Application error: {err}");
        process::exit(1);
    })
}
