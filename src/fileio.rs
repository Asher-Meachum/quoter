use std::env;
use rusqlite::Connection;

use std::fs::{self, DirBuilder};

use crate::text::Quote;

// Remove this function
fn home_dir_string() -> String {
    let home = env::home_dir().expect("Internal error: could not find home directory");
    home.display().to_string()
}

/// When called searches for the `~/.config/quoter` (the data directory) 
/// and if not present, attempts to make a directory at `~/.config/quoter`.
/// The data directory is not currently configureable at this time.
pub fn initialise() -> QuoteStorage {
    let path: String = format!("{}/{}", crate::fileio::home_dir_string(), ".config/quoter".to_string());
    let db: String = format!("{}/{}", path, "quotes.sqlite");

    data_dir_init(path);
    db_init(db)
}

fn data_dir_init(data_dir: String) {
    match fs::read_dir(data_dir.clone()) {
        Ok(_) => (),
        Err(_) => {
            DirBuilder::new()
                .recursive(true)
                .create(data_dir.clone())
                .expect("Internal error: couldn't initialise directory in ~/.config/quoter");
        },
    }
}

fn db_init(db_path: String) -> QuoteStorage {
    let db = Connection::open(&db_path).unwrap();
    db.table_exists(Some("main"), "quotes").unwrap();

    let columns: [bool; 4] = [
        db.column_exists(Some("main"), "quotes", "id").unwrap(),
        db.column_exists(Some("main"), "quotes", "title").unwrap(),
        db.column_exists(Some("main"), "quotes", "author").unwrap(),
        db.column_exists(Some("main"), "quotes", "content").unwrap(),
    ];

    for i in 0..columns.len() {
        if columns[i] {
            continue;
        } else {
            match i {
                0 => {db.execute("ALTER TABLE quotes ADD id PRIMARY KEY", ()).unwrap();},
                1 => {db.execute("ALTER TABLE quotes ADD title varchar(255)", ()).unwrap();},
                2 => {db.execute("ALTER TABLE quotes ADD author varchar(255)", ()).unwrap();},
                3 => {db.execute("ALTER TABLE quotes ADD content varchar(65535)", ()).unwrap();},
                _ => panic!("Internal logic error: columns.len() was set too high"),
            }
        }
    }
    QuoteStorage{db}
}

/// This struct is used for internal file operations.
/// To create an instance of this struct, use `DataFile::new_quote()`
pub struct QuoteStorage {
    db: Connection,
}

impl QuoteStorage {
    pub fn list(&self) -> Vec<String> {
        todo!("Implement list");
    }

    pub fn read(&self, name: String) -> Quote {
        todo!("Implement read");
    }
    
    pub fn add(&self, contents: Quote) {
        todo!("Implement write");
    }
}