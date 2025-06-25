use std::{env, error::Error, fs, process};

use minigrep::Config;
use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    //
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Oops: {err}");
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Opps: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}")
    }

    Ok(())
}
