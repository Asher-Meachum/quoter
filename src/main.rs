fn initialise() {
    use crate::file::DataDirectory;
    use std::fs::{DirBuilder, File};
    use std::fs;

    let path: String = DataDirectory::new(None).get_path();
    match fs::read_dir(path.clone()) {
        Ok(_) => (),
        Err(_) => {
            DirBuilder::new().recursive(true).create(path.clone()).expect("Internal error: couldn't initialise directory in ~/.config/quoter");
            File::create(format!("{}{}", path, "quotes.index")).expect("Internal error: Failed to create index file");
        },
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
        args::Arg::Generate => todo!(),
        args::Arg::Help => println!("{}", help_text()),
        args::Arg::Read(file) => println!("{}", file::read_quote(file)),
        args::Arg::List => {
            println!("Currently stored quotes are:");
            println!("{}", file::list_quotes())
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
    use std::fs::File;
    use std::io::{Read, Write};
    use std::env;
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[derive(Hash)]
    pub struct DataDirectory {
        name: Option<String>,
        path: String,
    }

    impl DataDirectory {
        pub fn new(file_name: Option<String>) -> DataDirectory {
            let data_path: String = format!("{}/.config/quoter", env::home_dir().expect("Internal error: failed getting home directory").to_string_lossy());
            DataDirectory{
                name: file_name,
                path: data_path,
            }
        }

        pub fn get_path(&self) -> String {
            format!("{}/{}",
             self.path, 
             match self.name.clone() {
                Some(name) => {
                    let mut file_hash = DefaultHasher::new();
                    name.hash(&mut file_hash);
                    format!("{}", file_hash.finish())
                },
                None => String::from(""),
            })
        }
    }

    pub fn list_quotes() -> String {
        match File::open(format!("{}{}", DataDirectory::new(None).get_path(), "quotes.index")) {
            Ok(mut index) => {
                let mut contents: String = String::new();
                index.read_to_string(&mut contents).expect("Internal error: Could not read index");
                contents
            }
            Err(_) => String::from("Internal error: index file not found."),
        }
    }

    pub fn read_quote(title: String) -> String {
        let file_path: String = DataDirectory::new(Some(title)).get_path();

        match File::open(file_path) {
            Ok(mut file) => {
                let mut contents: String = String::new();
                file.read_to_string(&mut contents).expect("Internal error: Could not read file");
                contents
            },
            Err(_) => String::from("Error: Quote file not found. Try quoter --list to see stored quotes"),
        }
    }

    pub fn write_quote(quote: crate::text::Quote) {
        let mut index: File = File::create(format!("{}{}", DataDirectory::new(None).get_path(), "quotes.index")).expect("Internal error: could not open index file");
        index.write_all(quote.title().as_bytes()).expect("Internal error: could not update index file");

        // Fix path traversal vulnerability from ..
        let file_path: String = DataDirectory::new(Some(quote.title())).get_path();
        let mut file: File = File::create(file_path).expect("Internal error: Failed to open file for writing.");
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