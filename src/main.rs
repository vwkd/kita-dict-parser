mod parser;
use parser::parser;

mod import;
use import::load_data;

fn main() {
    let s = load_data().unwrap();

    for line in s.lines() {
        println!("{:?}", parser(line).unwrap());
    }
}
