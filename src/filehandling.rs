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
