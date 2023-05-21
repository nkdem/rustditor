pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Option<Config> {
        args.next(); // Skip the first argument, which is the program name.
        return match args.next() {
            Some(filename) => Some(Config { filename }),
            None => None,
        }
    }
}