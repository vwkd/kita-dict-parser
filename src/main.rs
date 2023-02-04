mod parser;
mod import;

use parser::parser;
use import::load_data;

const NEXT_PAGE: &str = "1/39";

fn main() {
    let s = load_data(NEXT_PAGE).expect("Error loading data");
    
    for line in s.lines() {
        println!("{}", line);

        match parser::<nom::error::Error<&str>>(line) {
            Ok((_, entry)) => println!("{:?}", entry),
            Err(e) => eprintln!("{}", e),
        }

        println!("");
    }
}
