mod challenges;

use challenges::set1;
use challenges::set2;

use std::env;
use colored::Colorize;
use std::io::{self, Write};
use std::process::{Command, Stdio};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <challenge>", args[0]);
        eprintln!("Example: {} 1", args[0]);
        std::process::exit(1);
    }

    let challenge = &args[1];

    match challenge.as_str() {
        "0" => run_all_challenges_quiet(),

        // Set 1
        "1" => run_challenge("Set 1, Challenge 01", || set1::challenge01::run()),
        "2" => run_challenge("Set 1, Challenge 02", || set1::challenge02::run()),
        "3" => run_challenge("Set 1, Challenge 03", || set1::challenge03::run()),
        "4" => run_challenge("Set 1, Challenge 04", || set1::challenge04::run()),
        "5" => run_challenge("Set 1, Challenge 05", || set1::challenge05::run()),
        "6" => run_challenge("Set 1, Challenge 06", || set1::challenge06::run()),
        "7" => run_challenge("Set 1, Challenge 07", || set1::challenge07::run()),
        "8" => run_challenge("Set 1, Challenge 08", || set1::challenge08::run()),

        // Set 2
        "9" => run_challenge("Set 2, Challenge 09",  || set2::challenge09::run()),
        "10" => run_challenge("Set 2, Challenge 10", || set2::challenge10::run()),
        "11" => run_challenge("Set 2, Challenge 11", || set2::challenge11::run()),
        "12" => run_challenge("Set 2, Challenge 12", || set2::challenge12::run()),

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

fn run_all_challenges_quiet() {
    let challenges = vec![
        ("Set 1, Challenge 01", "1"),
        ("Set 1, Challenge 02", "2"),
        ("Set 1, Challenge 03", "3"),
        ("Set 1, Challenge 04", "4"),
        ("Set 1, Challenge 05", "5"),
        ("Set 1, Challenge 06", "6"),
        ("Set 1, Challenge 07", "7"),
        ("Set 1, Challenge 08", "8"),
        ("Set 2, Challenge 09", "9"),
        ("Set 2, Challenge 10", "10"),
        ("Set 2, Challenge 11", "11"),
        ("Set 2, Challenge 12", "12"),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (name, challenge_num) in challenges {
        print!("{}: ", name);
        io::stdout().flush().unwrap();
        
        // Run the challenge as a subprocess with suppressed output
        let output = Command::new(env::current_exe().unwrap())
            .arg(challenge_num)
            .stdout(Stdio::null())  // Suppress stdout
            .stderr(Stdio::null())  // Suppress stderr
            .status()
            .expect("Failed to run challenge");

        if output.success() {
            println!("{}", "✓ PASS".green());
            passed += 1;
        } else {
            println!("{}", "✗ FAIL".red());
            failed += 1;
        }
    }

    println!("\n{}: {} passed, {} failed", 
             "Summary".bold(), 
             passed.to_string().green(), 
             failed.to_string().red());

    if failed > 0 {
        std::process::exit(1);
    }
}