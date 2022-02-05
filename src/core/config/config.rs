#[derive(Debug)]
pub struct Config {
    pub debug: bool,
    pub secret_key: String,
}

impl Config {
    pub fn new() -> Self {
        let debug = dotenv::var("DEBUG")
            .expect("Can read the debug from env.")
            .parse::<bool>()
            .expect("Can parse the debug to bool");

        let secret_key = dotenv::var("SECRET_KEY").expect("Can read secret key from env.");

        Self { debug, secret_key }
    }
}
