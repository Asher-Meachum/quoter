use std::fmt;

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
    pub fn contents(&self) -> [String; 3] {
        [self.title.clone(), self.author.clone(), self.text.clone()]
    }

    pub fn new(title: String, author: String, text: String) -> Quote {
        Quote {
            title,
            author,
            text
        }
    }

    /// This method serves as an initialisaton for Quote.
    /// It takes the data and metadata fields directly from stdin, and handles user prompting.
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
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}", self.title, self.author, self.text)
    }
}

fn take_input() -> String {
    use std::io;
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Internal error: couldn't read input.");
    String::from(input.trim())
}