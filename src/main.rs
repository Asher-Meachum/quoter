use crate::text::Quote;
use rand::seq::IndexedRandom;

fn initialise() {
    use crate::file;
    use std::fs::{DirBuilder, File};
    use std::fs;

    let path: String = format!("{}/{}", file::home_dir_string(), ".config/quoter".to_string());
    match fs::read_dir(path.clone()) {
        Ok(_) => (),
        Err(_) => {
            DirBuilder::new().recursive(true).create(path.clone()).expect("Internal error: couldn't initialise directory in ~/.config/quoter");
            File::create(format!("{}/{}", path, "quotes.index")).expect("Internal error: Failed to create index file");
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
        args::Arg::ArgError(error) => println!("{}", error),
        args::Arg::Help => println!("{}", help_text()),
        args::Arg::Add => {
            let quote: text::Quote = Quote::new_from_input();
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
    use std::fs::OpenOptions;
    use std::io::{Read, Write};
    use std::env;
    use std::hash::{DefaultHasher, Hash, Hasher};

    pub fn home_dir_string() -> String {
        let home = env::home_dir().expect("Internal error: could not find home directory");
        home.display().to_string()
    }

    #[derive(Hash)]
    enum FileType{
        Index,
        Quote,
    }

    #[derive(Hash)]
    pub struct DataFile {
        filetype: FileType,
        name: String,
        path: String,
    }

    impl DataFile {
        pub fn new_quote(title: String) -> DataFile {
            let mut hash: DefaultHasher = DefaultHasher::new();
            title.hash(&mut hash);
            DataFile{filetype: FileType::Quote, name: title.clone(), path: format!("{}/{}/{}", home_dir_string(), ".config/quoter".to_string(), hash.finish().to_string())}
        }

        pub fn new_index() -> DataFile{
            DataFile{filetype: FileType::Index, name: "quotes.index".to_string(), path: format!("{}/{}", home_dir_string(), ".config/quoter/quotes.index".to_string())}
        }

        pub fn read(&self) -> String {
            match OpenOptions::new().read(true).open(self.path.clone()) {
                Ok(mut file) => {
                    let mut contents: String = String::new();
                    file.read_to_string(&mut contents).expect("Internal error: could not read file");
                    contents
                }
                Err(_) => "Error: could not read quote. Are you sure it exists (use quoter --list to check)? ".to_string()
            }
        }
        
        pub fn write(&self, contents: String) {
            let mut file: std::fs::File = OpenOptions::new().append(true).create(true).open(self.path.clone()).expect("Internal error: could not open file for writing");
            file.write_all(format!("{}\n", contents).as_bytes()).expect("Internal error: could not write to file")
        }
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