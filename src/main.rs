mod argparse;
mod fileio;
mod text;

use rand::seq::IndexedRandom;
use crate::{argparse::args, text::Quote};

fn help_text() -> &'static str {
"Usage: quoter [OPTIONS]

Options:
  -a, --add             Add a quote
  -d, --delete  <title> Delete the quote with title <title>
  -h, --help            Display this help message
  -l, --list            Display stored quote names
  -r, --read    <title> Read the quote with title <title>"
}

fn main() {
    match fileio::initialise() {
        Ok(storage) => {
            let storage = storage;
            match args::parse_args() {
                args::Arg::InvalidArg(error) => println!("{}", error),
                args::Arg::Help => println!("{}", help_text()),
                args::Arg::Add => {
                    let quote: Quote = match Quote::new_from_input() {
                        Ok(q) => q,
                        Err(_) => {
                            eprintln!("Error: could not read input. Retrying...\n");
                            Quote::new_from_input().expect("Retry failed. Exiting.")
                        }, 
                    };
                    match storage.add(quote) {
                        Ok(()) => println!("Successfully added quote."),
                        Err(e) => eprintln!("Error: could not add quote {:?}", e),
                    }
                },
                args::Arg::Generate => {
                    match storage.list() {
                        Ok(list) => {
                            match list.choose(&mut rand::rng()) {
                                Some(chosen) => {
                                    match storage.read(chosen.clone()) {
                                        Ok(quote) => println!("{}", quote),
                                        Err(_) => eprintln!("Error: could not read quote")
                                    }
                                },
                                None => println!("No files stored. Use quoter --add to add one.")
                            }
                        },
                        Err(e) => eprintln!("Error: could not retrieve random quote: {:?}", e),
                    };
                },
                args::Arg::List => {
                    match storage.list() {
                        Ok(list) => println!("{:?}", list),
                        Err(e) => eprintln!("Error: could not retrive quotes: {:?}", e)
                    };
                },
                args::Arg::Read(title) => {
                    match storage.read(title) {
                        Ok(quote) => println!("{}", quote),
                        Err(e) => eprintln!("Error: could not read quote: {:?}", e)
                    }
                },
                args::Arg::Delete(title) => {
                    match storage.delete(title.clone()) {
                        Ok(_) => println!("Successfully deleted quote \"{title}\"."),
                        Err(e) => println!("Error: could not delete quote \"{title}\": {:?}", e),
                    }
                },
            }
        },
        Err(e) => eprintln!("Error: could not initialise program: {:?}", e),
    }
}