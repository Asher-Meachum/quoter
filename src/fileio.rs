use std::env;
use std::io::{Read, Write};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::fs::{self, DirBuilder, File, OpenOptions};

fn home_dir_string() -> String {
    let home = env::home_dir().expect("Environment error: could not find home directory"); // This panic is intentional. The program cannot continue without knowing location of ~ and crashing here is safest and simplest place.
    home.display().to_string()
}

/// When called searches for the `~/.config/quoter` (the data directory) 
/// and if not present, attempts to make a directory at `~/.config/quoter`.
/// The data directory is not currently configureable at this time.
pub fn initialise() {
    let path: String = format!("{}/{}", crate::fileio::home_dir_string(), ".config/quoter".to_string());
    // Replace with proper function, path_exists() or something similar
    match fs::read_dir(path.clone()) {
        Ok(_) => (),
        Err(_) => {
            DirBuilder::new().recursive(true).create(path.clone())
                .expect("Internal error: couldn't initialise directory in ~/.config/quoter");
            File::create(format!("{}/{}", path, "quotes.index"))
                .expect("Internal error: Failed to create index file");
        },
    }
}

/// This struct is used for internal file operations.
/// To create an instance of this struct, use `DataFile::new_quote()`
#[derive(Hash)]
pub struct DataFile {
    name: String,
    path: String,
}

impl DataFile {
    /// This function creates a new instance of DataFile to be used for quote files.
    /// It takes the title of the quote, and uses it to complete the path at `~/.config/quoter/<file_name>`
    /// 
    /// Additionally, it completes the hashing of the title to get the file name, to prevent directory traversal.
    pub fn new_quote(title: String) -> DataFile {
        let mut hash: DefaultHasher = DefaultHasher::new();
        title.hash(&mut hash);
        DataFile{name: title.clone(), path: format!("{}/.config/quoter/{}", home_dir_string(), hash.finish().to_string())}
    }

    /// Initialises the index file with the correct name and path, `quotes.index` and `~/.config/quoter/quotes.index`, respectively.
    pub fn new_index() -> DataFile{
        DataFile{name: "quotes.index".to_string(), path: format!("{}/.config/quoter/quotes.index", home_dir_string())}
    }

    /// This method reads the data from a stored file, using the path field as the file path.
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
    
    /// This method writes the data the file, using the path field as the file path.
    /// It creates the file if it does not exist, and appends the data to the file.
    pub fn write(&self, contents: String) {
        let mut file: std::fs::File = OpenOptions::new().append(true).create(true).open(self.path.clone()).expect("Internal error: could not open file for writing");
        file.write_all(format!("{}\n", contents).as_bytes()).expect("Internal error: could not write to file")
    }
}