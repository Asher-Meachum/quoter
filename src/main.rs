mod argparse;
mod fileio;
mod text;

use rand::seq::IndexedRandom;
use crate::{argparse::args, fileio::DataFile, text::Quote};

// TODO: Use static &str instead of String
fn help_text() -> String {
    String::from(
"Usage: quoter [OPTIONS]

Options:
  -a, --add             Add a quote
  -h, --help            Display this help message
  -l, --list            Display stored quote names
  -r, --read <title>    Read the quote with title <title>"
    ) 
}

fn main() {
    fileio::initialise();

    match args::parse_args() {
        args::Arg::ArgError(error) => println!("{}", error),
        args::Arg::Help => println!("{}", help_text()),
        args::Arg::Add => {
            let quote: Quote = Quote::new_from_input();
            let quote_file: DataFile = DataFile::new_quote(quote.title());
            quote_file.write(quote.to_file_format());

            let index: DataFile = DataFile::new_index();
            index.write(quote.title());
        },
        args::Arg::Generate => {
            let index: DataFile = DataFile::new_index();
            let files: String = index.read();
            match files.trim().split("\n").collect::<Vec<&str>>().choose(&mut rand::rng()) {
                Some(chosen) => {
                    let quote: DataFile = DataFile::new_quote(String::from(*chosen));
                    println!("{}", quote.read());
                },
                None => println!("No files stored. Use quoter --add to add one.")
            };
        },
        args::Arg::List => {
            let index: DataFile = DataFile::new_index();
            println!("Your stored quotes are:\n{}", index.read())
        },
        args::Arg::Read(title) => {
            let quote: DataFile = DataFile::new_quote(title);
            println!("{}", quote.read());
        }
    }
}