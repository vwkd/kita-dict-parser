mod parser;
use parser::parser;

fn main() {
    let s = "აბასთუმანი (g. Kurort)";

    println!("{:?}", parser(s).unwrap());
}
