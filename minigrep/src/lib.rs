use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

#[derive(Debug)]
pub enum MinigrepError {
    NotImplemented,
    ArgCount(usize),
}

use MinigrepError::*;

impl fmt::Display for MinigrepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotImplemented => write!(f, "Not implemented"),
            ArgCount(n) => write!(f, "Expected 2 arguments, a query an a file. Got {}.", n),
        }
    }
}

impl Error for MinigrepError {}

pub fn run_query(conf: &Config) -> Result<(), Box<dyn Error>> {
    let fc = fs::read_to_string(&conf.filename)?;
    let r = search(&conf.query, &fc);

    if r.len() == 0 {
        println!("No results found");
    } else {
        for s in r {
            println!("{}", s);
        }
    }

    Ok(())
}

pub fn parse_config(mut args: env::Args) -> Result<Config, Box<dyn Error>> {
    // Skip the program name
    args.next();

    let qm = args.next();
    let fm = args.next();

    match (qm, fm) {
        (Some(q), Some(f)) => Ok(Config {
            query: q,
            filename: f,
        }),
        _ => Err(Box::new(ArgCount(args.len() - 1))),
    }
}

fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|l| l.contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_search() {
        let query = "duct";
        let text = "\
Rust is a very
productive language to work
in apparently.";
        assert_eq!(vec!["productive language to work"], search(query, text));
    }
}
