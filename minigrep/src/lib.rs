use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

pub fn run_query(conf: &Config) -> Result<&str, &str> {
    let fc = fs::read_to_string(&conf.filename)
        .expect(&format!("Unable to read file {}", conf.filename));

    Err("not done")
}
