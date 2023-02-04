mod import;
mod parser;

use import::load_data;
use parser::parser;

const NEXT_PAGE: &str = "1/39";

fn main() {
    let s = load_data(NEXT_PAGE).expect("Error loading data");

    // todo: handle skipped lines
    let lines = s.lines().enumerate().filter(|(i, l)| !l.contains("|"));

    for (index, line) in lines {
        let entry = parser::<nom::error::Error<&str>>(line);

        match entry {
            Err(e) => {
                eprintln!("{index}: {line}");
                eprintln!("{:?}\n", e);
            }
            Ok((_, entry)) => {
                println!("{index}: {line}");
                println!("{:?}\n", entry);
            }
        }
    }
}
