use std::env;

use rust_programs::{run_003, run_006, run_007};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "003" => {
            run_003::run();
        }
        "006" => {
            run_006::run();
        }
        "007" => {
            run_007::run();
        }
        _ => {
            println!("Invalid argument");
        }
    }
}
