mod parser;
mod import;

use parser::parser;
use import::load_data;

fn main() {
    let s = load_data().unwrap();
    
    for line in s.lines() {
        println!("{}", line);

        match parser::<nom::error::Error<&str>>(line) {
            Ok((_, entry)) => println!("{:?}", entry),
            Err(e) => eprintln!("{}", e),
        }

        println!("");
    }
}
