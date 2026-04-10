mod argparse;
mod fileio;
mod text;

use rand::seq::IndexedRandom;
use crate::{argparse::args, fileio::QuoteStorage, text::Quote};

// TODO: Use static &str instead of String
fn help_text() -> &'static str {
"Usage: quoter [OPTIONS]

Options:
  -a, --add             Add a quote
  -h, --help            Display this help message
  -l, --list            Display stored quote names
  -r, --read <title>    Read the quote with title <title>"
}

fn main() {
    let storage: QuoteStorage = fileio::initialise();

    match args::parse_args() {
        args::Arg::ArgError(error) => println!("{}", error),
        args::Arg::Help => println!("{}", help_text()),
        args::Arg::Add => {
            let quote: Quote = Quote::new_from_input();
            storage.add(quote);
        },
        args::Arg::Generate => {
            match storage.list().choose(&mut rand::rng()) {
                Some(chosen) => {
                    println!("{}", *chosen)
                },
                None => println!("No files stored. Use quoter --add to add one.")
            };
        },
        args::Arg::List => {
            println!("{:?}", storage.list());
        },
        args::Arg::Read(title) => {
            println!("{}", storage.read(title));
        }
    }
}