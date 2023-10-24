mod import;
mod parser;

use import::{load_dict, Entry};
use parser::{general, verb};

fn main() {
    let next_page = std::env::var("NEXT_PAGE").unwrap_or("1/39".to_owned());

    println!("Parsing dict until page {}...", next_page);

    let dict = load_dict(&next_page).expect("Error getting entries");

    for (index, entry) in dict.into_iter().enumerate() {
        match entry {
            Entry::Verb(ref line) => {
                continue;
                // println!("{index}: {line}");
                // let parsed_entry = verb::parse(line);
            }
            Entry::Other(ref line) => {
                println!("{index}: {line}");
                let parsed_entry = general::parse(line);
                println!("{:?}\n", parsed_entry);
            }
        }
    }
}
