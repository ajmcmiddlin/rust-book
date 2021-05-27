use minigrep;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query_result = parse_config(&args).and_then(|c| minigrep::run_query(&c));
    match query_result {
        Ok(r) => print!("{}", r),
        Err(e) => print!("{}", e),
    }
}

fn parse_config(args: &Vec<String>) -> Result<minigrep::Config, String> {
    let qm = args.get(1);
    let fm = args.get(2);

    match (qm, fm) {
        (Some(q), Some(f)) => Ok(minigrep::Config {
            query: q.to_string(),
            filename: f.to_string(),
        }),
        _ => Err(format!(
            "Expected 2 arguments: a query and a file. Got {} arguments.",
            args.len()
        )),
    }
}
