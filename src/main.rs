use clap::Parser;
use colored::Colorize;
use std::{fs::File, io::Read, path::Path};

#[derive(Parser)]
#[command(version)]
/// fsearch, search for a query in a file
struct UserInput {
    /// Path of the file
    #[arg(short, long, required = true)]
    file: Vec<String>,
    /// Query to search
    #[arg(short, long, required = true)]
    query: Vec<String>,
    /// Ignore case
    #[arg(short, long, default_value_t = false)]
    ignore_case: bool,
}

fn main() {
    let user_input = UserInput::parse();
    for path_string in &user_input.file {
        let path = Path::new(&path_string);
        match File::open(path) {
            Ok(mut file) => {
                for query in &user_input.query {
                    let mut file_content = String::new();
                    if let Err(e) = file.read_to_string(&mut file_content) {
                        eprintln!("{}", format!("Failed to read {}, {e}", path_string).red());
                    } else {
                        println!(
                            "{} file: {} query: {}",
                            "➔ ".to_string().green(),
                            path_string.blue().underline(),
                            query.blue().underline()
                        );
                        search_file(&file_content, query, user_input.ignore_case);
                    }
                }
            }
            Err(e) => eprintln!("{}", format!("Failed to open {}, {e}", path_string).red()),
        }
    }
}

fn search_file(file_content: &str, query: &str, ignore_case: bool) {
    if ignore_case {
        case_insensitive_search(file_content, query);
    } else {
        case_sensitive_search(file_content, query);
    }
}

fn case_insensitive_search(file_content: &str, query: &str) {
    let query = query.to_lowercase();
    for (line_number, line) in file_content.lines().enumerate() {
        if let Some(index) = (line.to_lowercase()).find(&query) {
            let query_len = query.len();
            print_matched_line(line, line_number, index, query_len);
        }
    }
}

fn case_sensitive_search(file_content: &str, query: &str) {
    for (line_number, line) in file_content.lines().enumerate() {
        if let Some(index) = line.find(query) {
            let query_len = query.len();
            print_matched_line(line, line_number, index, query_len);
        }
    }
}

fn print_matched_line(line: &str, line_number: usize, index: usize, query_len: usize) {
    let (start_string, remaining_string) = line.split_at(index);
    let (middle_string, end_string) = remaining_string.split_at(query_len);
    println!(
        "{}: {}{}{}",
        (line_number + 1).to_string().blue(),
        start_string,
        middle_string.purple().bold(),
        end_string,
    );
}
