use std::{env, process};
use temperature::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err (err) = run(config){
        eprintln!("Problem converting arguments: {}", err);
        process::exit(1);
    }
}
