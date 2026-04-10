use std::env;
use rusqlite::{Connection, Error};
use std::fs::{self, DirBuilder};

use crate::text::Quote;

/// Custom error type to simplify errors from `rusqlite::Error`
#[derive(Debug)]
pub enum StorageError {
    ReadError,
    WriteError,
    QueryError,
    UnknownError,
}

impl From<rusqlite::Error> for StorageError {
    fn from(error: rusqlite::Error) -> Self {
        match error {
            Error::SqliteFailure(..) => Self::QueryError,
            Error::InvalidParameterName(..) => Self::QueryError,
            Error::QueryReturnedNoRows => Self::QueryError,
            Error::QueryReturnedMoreThanOneRow => Self::QueryError,
            Error::InvalidQuery => Self::QueryError,
            Error::MultipleStatement => Self::QueryError,
            Error::InvalidColumnIndex(..) => Self::ReadError,
            Error::InvalidColumnName(..) => Self::ReadError,
            Error::StatementChangedRows(..) => Self::WriteError,
            _ => Self::UnknownError,
        }
    }
}

/// Serves as an orchestrator of environment initialisation. For more details,
/// see `data_dir_init()` and `db_init()`.
pub fn initialise() -> Result<QuoteStorage, StorageError> {
    let path: String = format!(
        "{}/{}", 
        env::home_dir().expect("Internal error: could not find home directory").display().to_string(),
         ".config/quoter".to_string()
    );
    let db: String = format!("{}/{}", path, "quotes.sqlite");

    data_dir_init(path);
    db_init(db)
}

/// When called searches for the `~/.config/quoter` (the data directory) 
/// and if not present, attempts to make a directory at `~/.config/quoter`.
/// The data directory is not currently configureable at this time.
fn data_dir_init(data_dir: String) {
    match fs::read_dir(&data_dir) {
        Ok(_) => (),
        Err(_) => {
            DirBuilder::new()
                .recursive(true)
                .create(&data_dir)
                .expect("Error: couldn't initialise directory in ~/.config/quoter");
        },
    }
}

/// When called, it checks for database existance in the data directory.
/// If a database exists, it validates the schema, and corrects it if incorrect.
/// If a database does not exist, it creates one in that directory.
/// It then returns the database connection.
fn db_init(db_path: String) -> Result<QuoteStorage, StorageError> {
    let db = Connection::open(&db_path)?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS quotes (
                id INT PRIMARY KEY, 
                title TEXT NOT NULL UNIQUE,
                author TEXT,
                content TEXT)", 
            ()
        )?;

    let columns: [&'static str; 3] = ["title", "author", "content"];

    for column in columns {
        if ! db.column_exists(Some("main"), "quotes", column)? {
            db.execute("ALTER TABLE quotes ADD ?1 TEXT", [column])?;
        }
    }

    Ok(QuoteStorage{db})
}

/// Container for the sqlite connection and commands.
/// This struct cannot initialise itself. To use this,
/// struct, use the return type from `initialise()`
/// ## Quickstart
/// ```
/// let storage: QuoteStorage = initialise().unwrap();
/// storage.list().unwrap();
/// ```
pub struct QuoteStorage {
    db: Connection,
}

impl QuoteStorage {
    /// Lists all quotes stored in the database using the title field.
    /// This function returns a Result<> value, however, it may panic if
    /// retrieving the title from the SQLite response raises an error.
     pub fn list(&self) -> Result<Vec<String>, StorageError> {
        let mut stmt = self.db.prepare("SELECT title FROM quotes")?;
        let titles = stmt.query_map(
            (), 
            |row| row.get(0),
        )?;
        Ok(titles.map(|x| x.unwrap()).collect())
    }

    /// Reads a quote with the given title.
    /// Raises a `StorageError::ReadError` if the quote with selected title
    /// does not exist.
    pub fn read(&self, name: String) -> Result<Quote, StorageError> {
        let mut stmt = self.db.prepare("SELECT title, author, content FROM quotes WHERE title = ?1")?;
        let mut quotes = stmt.query_map(
            [name], 
            |row| Ok(Quote::new(row.get(0)?, row.get(1)?, row.get(2)?))
        )?;

        match quotes.next() {
            Some(column_content) => {
                match column_content {
                    Ok(quote) => Ok(quote),
                    Err(e) => Err(e.into()),
                }
            },
            None => Err(StorageError::ReadError),
        }
    }
    
    /// Adds a quote to the database and returns unit type if successful.
    /// This function is designed such that as long as schema passes
    /// `initialisation()`, the schema should not cause a panic.
    pub fn add(&self, contents: Quote) -> Result<(), StorageError> {
        let quote = contents.contents();
        self.db.execute(
            "INSERT INTO quotes (title, author, content) VALUES (?1, ?2, ?3)", 
            (quote[0].clone(), quote[1].clone(), quote[2].clone())
        )?;

        Ok(())
    }

    /// Deletes a quote. This function will report success even for the deletion of
    /// nonexistent quotes.
    pub fn delete(&self, title: String) -> Result<(), StorageError> {
        self.db.execute("DELETE FROM quotes WHERE title = ?1", [title])?;
        Ok(())
    }
}