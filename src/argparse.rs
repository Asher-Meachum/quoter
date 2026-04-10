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
