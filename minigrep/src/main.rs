use minigrep;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query_result = minigrep::parse_config(&args).and_then(|c| minigrep::run_query(&c));
    if let Err(e) = query_result {
        print!("{}", e)
    }
}
