use std::env;
use minigrep::Config;

fn main() {
    let args  = env::args().collect::<Vec<_>>();
    let config = Config::new(&args).expect("Config parse error");

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("Sensitive mode: {}\n", if config.is_case_sensitive {"ON"} else {"OFF"});

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
    }
}