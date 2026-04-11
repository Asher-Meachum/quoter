//! Module for parsing command line arguments
//!
//! args provides a simple API to retrieve command line arguments
//! ### Quickstart
//! ```
//! mod args;
//! use crate::argparse::args;
//! let arg_result: args::Arg = args::parse_args();
//! ```

pub mod args {
    use std::env;

    pub enum Arg {
        Add,
        InvalidArg(&'static str),
        Delete(String),
        Generate,
        Help,
        List,
        Read(String),
    }

    /// Returns the argument as an enum option from `args::Arg`.
    /// 
    /// ### Errors
    /// InvalidArg is used as a catch-all for missing/invalid arguments.
    /// It must be noted that non-UTF-8 CLI args will cause a panic. This is a limitation of
    /// `std::env::args()`.
    pub fn parse_args() -> Arg {
        let args: Vec<String> = env::args().collect();

        match args.get(1) {
            Some(arg) => {
                match arg as &str {
                    "add" => Arg::Add,
                    "read" => {
                        match args.get(2..) {
                            Some(file) => {
                                Arg::Read(file.join(" ".trim()))
                            },
                            None => Arg::InvalidArg("Error: quote title not provided"),
                        } 
                    },
                    "delete" => {
                        match args.get(2) {
                            Some(file) => Arg::Delete(file.to_string()),
                            None => Arg::InvalidArg("Error: quote title not provided"),
                        }
                    },
                    "-h" | "--help" => Arg::Help,
                    "list" => Arg::List,
                    _ => Arg::InvalidArg("Error: Unknown flag"),
                }
            },
            None => Arg::Generate,
        }
    }
}
