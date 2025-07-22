mod challenges;

use challenges::set1;
use challenges::set2;

use std::env;
use colored::Colorize;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <challenge>", args[0]);
        eprintln!("Example: {} 1", args[0]);
        std::process::exit(1);
    }

    let challenge = &args[1];

    match challenge.as_str() {
        // Set 1
        "1" => run_challenge("Set 1, Challenge 01", || set1::challenge01::run()),
        "2" => run_challenge("Set 1, Challenge 02", || set1::challenge02::run()),
        "3" => run_challenge("Set 1, Challenge 03", || set1::challenge03::run()),
        "4" => run_challenge("Set 1, Challenge 04", || set1::challenge04::run()),
        "5" => run_challenge("Set 1, Challenge 05", || set1::challenge05::run()),
        "6" => run_challenge("Set 1, Challenge 06", || set1::challenge06::run()),
        "7" => run_challenge("Set 1, Challenge 07", || set1::challenge07::run()),

        // Set 2
        "9" => run_challenge("Set 2, Challenge 07",  || set2::challenge09::run()),
        _ => {
            eprintln!("Unknown challenge: Challenge {}", challenge);
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