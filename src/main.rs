use std::process;
use pic_man::*;
use structopt::StructOpt;

fn main() {
    run(Commands::from_args()).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        process::exit(1);
    });
}