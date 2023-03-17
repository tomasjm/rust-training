use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Error en la aplicación: {}", e);
        process::exit(1);
    }

}

