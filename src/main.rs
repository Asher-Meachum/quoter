mod argparse;
mod filehandling;
mod texthandling;

use rand::seq::IndexedRandom;
use std::fs::{self, DirBuilder, File};
use crate::{argparse::args, filehandling::file, texthandling::text};

fn initialise() {
    let path: String = format!("{}/{}", file::home_dir_string(), ".config/quoter".to_string());
    match fs::read_dir(path.clone()) {
        Ok(_) => (),
        Err(_) => {
            DirBuilder::new().recursive(true).create(path.clone()).expect("Internal error: couldn't initialise directory in ~/.config/quoter");
            File::create(format!("{}/{}", path, "quotes.index")).expect("Internal error: Failed to create index file");
        },
    }
}

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
    initialise();

    match args::parse_args() {
        args::Arg::ArgError(error) => println!("{}", error),
        args::Arg::Help => println!("{}", help_text()),
        args::Arg::Add => {
            let quote: text::Quote = text::Quote::new_from_input();
            let quote_file: file::DataFile = file::DataFile::new_quote(quote.title());
            quote_file.write(quote.to_file_format());

            let index: file::DataFile = file::DataFile::new_index();
            index.write(quote.title());
        },
        args::Arg::Generate => {
            let index: file::DataFile = file::DataFile::new_index();
            let files: String = index.read();
            match files.trim().split("\n").collect::<Vec<&str>>().choose(&mut rand::rng()) {
                Some(chosen) => {
                    let quote: file::DataFile = file::DataFile::new_quote(String::from(*chosen));
                    println!("{}", quote.read());
                },
                None => println!("No files stored. Use quoter --add to add one.")
            };
        },
        args::Arg::List => {
            let index: file::DataFile = file::DataFile::new_index();
            println!("Your stored quotes are:\n{}", index.read())
        },
        args::Arg::Read(title) => {
            let quote: file::DataFile = file::DataFile::new_quote(title);
            println!("{}", quote.read());
        }
    }
}