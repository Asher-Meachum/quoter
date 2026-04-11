use std::{fmt, io};

/// Quote is an internal struct used to store both quote data and metadata.
/// For a Quote to be stored in the database, the title field must not be empty.
/// ### Quickstart
/// You can construct a Quote interactively with the following:
/// ```
/// let quote: Quote = Quote::new_from_input();
/// ```
/// Alternatively, if you need to construct it from pre-existing data, you can use `Quote::new()`
/// ```
/// let quote: Quote = Quote::new(title, author, text);
/// ```
pub struct Quote {
    title: String,
    author: String,
    text: String,
}

impl Quote {
    /// Returns the fields of the Quote as an ordered
    /// array of String. The order is as follows: title, author, text 
    pub fn contents(&self) -> [String; 3] {
        [self.title.clone(), self.author.clone(), self.text.clone()]
    }

    /// Static constructor for Quote. For dynamic construction
    /// from user input, use `Quote::new_from_input()`
    pub fn new(title: String, author: String, text: String) -> Quote {
        Quote {
            title,
            author,
            text
        }
    }

    /// This method serves as an initialisaton for Quote.
    /// It takes the data and metadata fields directly from stdin, and handles user prompting.
    /// For construction from known data, use `Quote::new()`
    pub fn new_from_input() -> Result<Quote, io::Error> {
        println!("Enter the title of the quote:");
        let user_title: String = take_input()?; // TODO: Make field required
        println!("Enter the author of the quote (optional):");
        let user_author: String = take_input()?;
        println!("Enter the text of the quote:");
        let user_text: String = take_input()?;

        let quote = Quote {
            title: user_title,
            author: user_author,
            text: user_text,
        };
        Ok(quote)
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}", self.title, self.author, self.text)
    }
}

/// Private helper function for dynamic Quote construction
fn take_input() -> Result<String, io::Error> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    Ok(input)
}