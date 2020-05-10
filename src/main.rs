use std::error::Error;
use std::io;
use std::io::BufRead;
use std::process;

fn main() {
    let stdin = io::stdin();

    let stdin = stdin.lock().lines().map(|line| {
        line.unwrap_or_else(|err| {
            let cause = err.description();
            eprintln!("I/O error reading from stdin: {}", cause);
            process::exit(1);
        })
    });

    let sum = stdio_utils::sum(stdin).unwrap_or_else(|err| {
        eprintln!("Error during operation: {:?}", err);
        process::exit(1);
    });

    println!("{}", sum);
}
