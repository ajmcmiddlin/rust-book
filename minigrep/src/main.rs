use minigrep;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_config(&args) {
        Ok(config) => minigrep::run_query(&config),
        Err(e) => panic!(e)
    }
}

fn parse_config(args: &Vec<String>) -> Result<minigrep::Config, &str> {
    let qm = args.get(1);
    let fm = args.get(2);

    match (qm, fm) {
        (Some(q), Some(f)) => 
            Ok(minigrep::Config {query: q.to_string(), filename: f.to_string()})
        ,
        (None, None) => Err("Expected 2 arguments: query and file"),
        (Some(_q), None) => Err("Expected 2 arguments. Given a query but no file"),
        (None, Some(_f)) => Err("Expected 2 arguments. Somehow given a file (second argument) but not a query (first argument)"),
    }
}
