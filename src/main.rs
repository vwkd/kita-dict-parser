mod import;
mod parser;

use colored::Colorize;
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
            Entry::Other(line) => match general::parse(line) {
                Ok(general::Entry(
                    general::term::Term(term),
                    general::expression::Expression(expr),
                )) => {
                    println!("{}: {: <30} {}", index.to_string().green(), term, expr);
                }
                Err(err) => {
                    println!("{}: {}", index.to_string().red(), err.bold());
                }
            },
        }
    }
}
