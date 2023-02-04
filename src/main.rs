mod parser;
mod import;

use parser::parser;
use import::load_data;

fn main() {
    let s = load_data().unwrap();

    
    for line in s.lines() {
        let (_, entry) = parser::<()>(line).unwrap();

        println!("{:?}", entry);
    }
}
