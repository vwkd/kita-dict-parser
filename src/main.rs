mod import;
mod parser;

use import::{load_dict, Entry};
use parser::{general, verb};

fn main() {
    let next_page = std::env::var("NEXT_PAGE");
    let next_page = next_page.as_deref().unwrap_or("1/39");

    println!("Parsing dict until page {}...", next_page);

    let mut dict = load_dict(next_page).expect("Error getting entries");

    for (index, entry) in dict.iter_mut().enumerate() {
        match entry {
            Entry::Verb(line) => {
                // println!("{index}: {line}");
                // let parsed_entry = verb::parse(line);
                // println!("{:?}\n", parsed_entry);
            }
            Entry::Other(line) => {
                println!("{index}: {line}");
                let parsed_entry = general::parse(line);
                println!("{:?}\n", parsed_entry);
            }
        }
    }
}
