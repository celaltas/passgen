mod cli;
mod pasword;

use cli::Cli;
use pasword::generate_password;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli: Cli = Cli::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    if cli.show_help {
        println!("{}", Cli::help());
        process::exit(0)
    }
    let password = generate_password(&cli);
    println!("password: {}", password)
}
