fn main() {
    todo!()
}

fn _get_input() {}

pub mod args {
    use std::env;
    use crate::file;
    fn collect_arg() -> Vec<String> {
        let arguments: Vec<String> = env::args().collect();
        arguments
    }

    pub fn parse_args() -> (file::Action, String) {
        let args: Vec<String> = collect_arg();

        let help_message: &str = 
"This is quoter, a CLI tool to record and bring back quotes.

Usage: quoter [<arg>] filename

Options supported in this version: 
    -a, --add: Add a new quote. 
    -r, --read <title>: Read a specific quote. The quote can be accessed using the Title
    -h, --help: Display this help.";

        match args.get(1) {
            Some(arg) => {
                match arg as &str {
                    "-a" | "--add" => (file::Action::Add, String::from("")),
                    "-r" | "--read" => {
                        let identifier: String = match args.get(2) {
                            Some(arg) => arg.clone(),
                            None => String::from("")
                        };
                        return (file::Action::Read, identifier)
                        },
                    "-h"| "--help" => (file::Action::NoAct, String::from(help_message)),
                    _ => (file::Action::NoAct, String::from("Invalid Argument. Exiting.")),
                }
            },
            None => (file::Action::Generate, String::from("")),
        }
    }
}
pub mod file {
    #[derive(Debug)]
    pub enum Action {
        Add,
        Generate,
        Read,
        NoAct,
    }
}