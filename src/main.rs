use std::env;

use lr_grustep::grep;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <file_path>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let file_path = &args[2];

    match grep(pattern, file_path) {
        Ok(matched_lines) => {
            for line in matched_lines {
                println!("{}", line);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
