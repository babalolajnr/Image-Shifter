pub struct Config {
    pub input: String,
    pub action: String,
}

impl Config {
    pub fn new(input: String, action: String) -> Self {
        Self { input, action }
    }
}
