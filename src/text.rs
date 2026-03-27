use std::io;

/// Quote is an internal struct used to store both quote data and metadata.
/// At this time, you can only initialise a Quote with data from stdin.
/// ### Quickstart
/// ```
/// let quote; Quote = Quote::new_from_input();
/// ```
pub struct Quote {
    title: String,
    author: String,
    text: String,
}

impl Quote {
    /// This method serves as an initialisaton for Quote.
    /// It takes the data and metadata fields directly from stdin, and handles user prompting.
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

    /// This methods serialises the data into the format used for storage.
    /// The serialisation places each struct field on a new line.
    pub fn to_file_format(&self) -> String {
        format!("{}\n{}\n{}", self.title, self.author, self.text)
    }

    /// Returns the title field of the struct as an owned String.
    pub fn title(&self) -> String {
        self.title.clone()
    }
}

fn take_input() -> Result<String, io::Error> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    Ok(input)
}