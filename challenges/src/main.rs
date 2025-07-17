mod challenges;

use challenges::set1;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <set> <challenge>", args[0]);
        eprintln!("Example: {} 1 1");
        std::process::exit(1);
    }

    let set = &args[1];
    let challenge = &args[2];

    match (set.as_str(), challenge.as_str()) {
        ("1", "1") => run_challenge("Set 1, Challenge 01", set1::challenge01::run()),
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
        println!("✓ PASS");
    } else {
        println!("✗ FAIL");
        std::process::exit(1);
    }
}