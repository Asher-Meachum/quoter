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
        db.execute(
            "CREATE TABLE IF NOT EXISTS quotes (
                id INT PRIMARY KEY, 
                title TEXT NOT NULL UNIQUE,
                author TEXT,
                content TEXT)", 
            ()
        ).unwrap();

    let columns: [&'static str; 3] = ["title", "author", "content"];

    for column in columns {
        if ! db.column_exists(Some("main"), "quotes", column).unwrap() {
            db.execute("ALTER TABLE quotes ADD ?1 TEXT", [column]).unwrap();
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
        let mut stmt = self.db.prepare("SELECT title FROM quotes").unwrap();
        let titles = stmt.query_map(
            (), 
            |row| row.get(0),
        ).unwrap();
        titles.map(|x| x.unwrap()).collect()
    }

    pub fn read(&self, name: String) -> Quote {
        let mut stmt = self.db.prepare("SELECT title, author, content FROM quotes WHERE title = ?1").unwrap();
        let mut quotes = stmt.query_map(
            [name], 
            |row| Ok(Quote::new(row.get(0).unwrap(), row.get(1).unwrap(), row.get(2).unwrap()))
        ).unwrap();

        let quote: Quote = match quotes.next() {
            Some(column_content) => column_content.unwrap(),
            None => panic!(),
        };
        
        quote
    }
    
    pub fn add(&self, contents: Quote) {
        let quote = contents.contents();
        self.db.execute(
            "INSERT INTO quotes (title, author, content) VALUES (?1, ?2, ?3)", 
            (quote[0].clone(), quote[1].clone(), quote[2].clone())
        ).unwrap();
    }

    pub fn delete(&self, title: String) {
        self.db.execute("DELETE FROM quotes WHERE title = ?1", [title]).unwrap();
    }
}