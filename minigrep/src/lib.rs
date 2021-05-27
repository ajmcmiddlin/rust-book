use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run_query(conf: &Config) -> Result<String, String> {
    let fc = fs::read_to_string(&conf.filename)
        .expect(&format!("Unable to read file {}", conf.filename));

    Err("not done".to_string())
}
