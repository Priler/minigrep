pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub is_case_sensitive: bool
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
        {
            // validate
            if args.len() < 3 {Err("Not enough arguments.")} else {Ok(())}
        }?;

        let query = &args[1];
        let filename = &args[2];
        let is_case_sensitive = if args.len() >= 4 {!args[3].ends_with("-i")} else {true};

        Ok(Config {query, filename, is_case_sensitive})
    }
}
