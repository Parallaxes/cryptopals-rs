mod challenges;

use challenges::set1;

use std::env;
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <set> <challenge>", args[0]);
        eprintln!("Example: {} 1 1", args[0]);
        std::process::exit(1);
    }

    let set = &args[1];
    let challenge = &args[2];

    match (set.as_str(), challenge.as_str()) {
        ("1", "1") => run_challenge("Set 1, Challenge 01", || set1::challenge01::run()),
        ("1", "2") => run_challenge("Set 1, Challenge 02", || set1::challenge02::run()),
        ("1", "3") => run_challenge("Set 1, Challenge 03", || set1::challenge03::run()),
        ("1", "4") => run_challenge("Set 1, Challenge 04", || set1::challenge04::run()),
        ("1", "5") => run_challenge("Set 1, Challenge 05", || set1::challenge05::run()),
        ("1", "6") => run_challenge("Set 1, Challenge 06", || set1::challenge06::run()),
        _ => {
            eprintln!("Unknown challenge: Set {} Challenge {}", set, challenge);
            eprintln!("Available challenges:");
            eprintln!("  Set 1: 1, 2, ...");
            std::process::exit(1);
        }
    }
}

fn run_challenge<F>(name: &str, challenge_fn: F) 
where 
    F: FnOnce() -> bool 
{
    print!("{}: ", name);
    if challenge_fn() {
        println!("{}", "✓ PASS".green());
    } else {
        println!("{}", "✗ FAIL".red());
        std::process::exit(1);
    }
}