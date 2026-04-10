//! Module for parsing command line arguments
//!
//! args provides a simple API to retrieve command line arguments
//! ### Quickstart
//! ```
//! mod args;
//! use crate::args::args as args;
//! let arg_result: args::Arg = args::parse_args();
//! ```

pub mod args {
    use std::env;

    pub enum Arg {
        Add,
        ArgError(String),
        Delete(String),
        Generate,
        Help,
        List,
        Read(String),
    }

    fn collect_arg() -> Vec<String> {
        let arguments: Vec<String> = env::args().collect();
        arguments
    }

    /// Returns the argument as an enum option from `args::Arg`.
    /// 
    /// ArgError is returned as a general catch all for unexpected arguments.
    pub fn parse_args() -> Arg {
        let args: Vec<String> = collect_arg();

        match args.get(1) {
            Some(arg) => {
                match arg as &str {
                    "-a" | "--add" => Arg::Add,
                    "-r" | "--read" => {
                        match args.get(2) {
                            Some(file) => Arg::Read(file.to_string()),
                            None => Arg::ArgError("Error: quote title not provided".to_string()),
                        } 
                    },
                    "-d" | "--delete" => {
                        match args.get(2) {
                            Some(file) => Arg::Delete(file.to_string()),
                            None => Arg::ArgError("Error: quote title not provided".to_string()),
                        }
                    },
                    "-h" | "--help" => Arg::Help,
                    "-l" | "--list" => Arg::List,
                    _ => Arg::ArgError(String::from("Error: Unknown flag")),
                }
            },
            None => Arg::Generate,
        }
    }
}
