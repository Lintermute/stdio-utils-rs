use std::io::BufRead;
use stdio_utils::ApplicationError;

fn main() {
    let stdin  = std::io::stdin();
    let result = stdio_utils::sum(stdin.lock().lines());

    let sum = result.unwrap_or_else(|err| match err {
        ApplicationError::InputError(e) => {
            eprintln!("I/O error reading from stdin: {}", e);
            std::process::exit(1)
        }
        ApplicationError::ParsingError(e) => {
            eprintln!("Bad input data: {}", e);
            std::process::exit(1)
        },
    });

    println!("{}", sum);
}
