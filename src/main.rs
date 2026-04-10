use rand::seq::IndexedRandom;

fn initialise() {
    use crate::file;
    use std::fs;

    let path: String = file::data_dir();
    match fs::read_dir(path.clone()) {
        Ok(_) => (),
        Err(_) => fs::DirBuilder::new().recursive(true).create(path).expect("Internal error: couldn't initialise directory in ~/.config/quoter"),
    }
}

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
    use crate::{args, file, text};

    initialise();

    match args::parse_args() {
        args::Arg::Add => file::write_quote(text::Quote::new_from_input()),
        args::Arg::Generate => {
            match file::list_files().choose(&mut rand::rng()) {
                Some(chosen_file) => {
                    println!("{}", file::read_quote(chosen_file.clone()));
                },
                None => println!("No quotes stored. Use quoter --add to add one")
            }
        },
        args::Arg::Help => println!("{}", help_text()),
        args::Arg::Read(file) => println!("{}", file::read_quote(file)),
        args::Arg::List => {
            println!("Currently stored quotes are:");
            for file in file::list_files() {
                println!("{}", file)
            }
        },
        args::Arg::ArgError(error) => println!("{}", error),
    }
}

pub mod args {
    use std::env;

    pub enum Arg {
        Add,
        Help,
        ArgError(String),
        Read(String),
        Generate,
        List,
    }

    fn collect_arg() -> Vec<String> {
        let arguments: Vec<String> = env::args().collect();
        arguments
    }

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
                    "-h" | "--help" => Arg::Help,
                    "-l" | "--list" => Arg::List,
                    _ => Arg::ArgError(String::from("Error: Unknown flag")),
                }
            },
            None => Arg::Generate,
        }
    }
}

pub mod file {
    use std::{fs::{self, DirEntry, File}};
    use std::io::{Read, Write};
    use std::env;

    pub fn data_dir() -> String {
        return format!("{}/.config/quoter", env::home_dir().expect("Internal error: failed getting home directory").to_string_lossy());
    }

    pub fn list_files() -> Vec<String> {
        let path: String = data_dir();
        let mut file_paths: Vec<DirEntry> = Vec::new();
        for file in fs::read_dir(path).unwrap() {
            file_paths.push(file.unwrap())
        }
        let mut files: Vec<String> = Vec::new();
        for file in file_paths {
            files.push(file.path().file_name().unwrap().to_string_lossy().into_owned());
        }
        files
    }

    pub fn read_quote(title: String) -> String {
        let path: String = format!("{}/", data_dir());

        match File::open(format!("{}{}.txt", path, title)) {
            Ok(mut file) => {
                let mut contents: String = String::new();
                file.read_to_string(&mut contents).expect("Internal error: Could not read file");
                contents
            },
            Err(_) => String::from("Error: Quote file not found. Try quoter --list to see stored quotes"),
        }
    }

    pub fn write_quote(quote: crate::text::Quote) {
        // Fix path traversal vulnerability from ..
        let path: String = format!("{}/", data_dir());
        let mut file: File = File::create(format!("{}{}.txt", path, quote.title())).expect("Internal error: Failed to open file for writing.");
        file.write_all(quote.to_file_format().as_bytes()).expect("Internal error: Failed to write to file")
    }
}

pub mod text {
    pub struct Quote {
        title: String,
        author: String,
        text: String,
    }

    impl Quote {
        pub fn new_from_input() -> Quote {
            println!("Enter the title of the quote:");
            let user_title: String = take_input(); // TODO: Make field required
            println!("Enter the author of the quote (optional):");
            let user_author: String = take_input();
            println!("Enter the text of the quote:");
            let user_text: String = take_input();
            Quote {
                title: user_title,
                author: user_author,
                text: user_text,
            }
        }

        pub fn to_file_format(&self) -> String{
            format!("{}\n{}\n{}", self.title, self.author, self.text)
        }

        pub fn title(&self) -> String {
            self.title.clone()
        }
    }

    fn take_input() -> String {
        use std::io;
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Internal error: couldn't read input.");
        String::from(input.trim())
    }
}