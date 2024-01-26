use clap::{App, Arg};
use lr_grustep::{grep, grep_count, grep_recursive, GrepOptions, MatchResult};

fn main() {
    // Define command-line arguments using clap
    let matches = App::new("grustep")
        .version("0.1.0")
        .author("literank")
        .about("A grep-like utility in Rust")
        .arg(Arg::with_name("pattern").required(true).index(1).help("The pattern to search for"))
        .arg(Arg::with_name("file_path").required(false).index(2).help("The file to search in"))
        .arg(Arg::with_name("count").short("c").long("count").help("Only a count of selected lines is written to standard output"))
        .arg(Arg::with_name("ignore-case").short("i").long("ignore-case").help("Perform case-insensitive matching"))
        .arg(Arg::with_name("line-number").short("n").long("line-number").help("Each output line is preceded by its relative line number in the file, starting at line 1"))
        .arg(Arg::with_name("recursive").short("r").long("recursive").help("Recursively search subdirectories listed"))
        .arg(Arg::with_name("invert-match").short("v").long("invert-match").help("Selected lines are those not matching any of the specified patterns"))
        .get_matches();

    // Extract command-line arguments
    let pattern = matches.value_of("pattern").unwrap();
    let file_path = matches.value_of("file_path").unwrap_or("");
    let options = GrepOptions {
        ignore_case: matches.is_present("ignore-case"),
        invert_match: matches.is_present("invert-match"),
    };

    let result = if matches.is_present("recursive") && !file_path.is_empty() {
        grep_recursive(pattern, file_path.as_ref(), &options)
    } else {
        grep(pattern, file_path.as_ref(), &options)
    };

    match result {
        Ok(result) => {
            if matches.is_present("count") {
                println!("{}", grep_count(&result));
            } else {
                print_result(&result, matches.is_present("line-number"))
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

fn print_result(result: &MatchResult, show_line_number: bool) {
    let mut current_file = "";
    let file_count = result.len();

    for (file_path, items) in result {
        for item in items {
            if file_count > 1 && file_path != current_file {
                current_file = &file_path;
                println!("\n{}:", current_file);
            }
            if show_line_number {
                println!("{}: {}", item.line_number, item.line);
            } else {
                println!("{}", item.line);
            }
        }
    }
}
