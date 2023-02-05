mod import;
mod parser;

use import::{load_data, DictData};
use parser::general;
use parser::verb;

const NEXT_PAGE: &str = "1/39";

fn main() {
    let DictData(lines_noverbs, lines_verbs) = load_data(NEXT_PAGE).expect("Error loading data");

    for (index, line) in lines_noverbs.into_iter().enumerate() {
        let entry = general::parser::<nom::error::Error<&str>>(&line);

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

    for (index, line) in lines_verbs.into_iter().enumerate() {
        let entry = verb::parser::<nom::error::Error<&str>>(&line);

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
