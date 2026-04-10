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