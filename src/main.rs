mod import;
mod parser;

use import::{load_data, DictData};
use parser::general;
use parser::verb;

const NEXT_PAGE: &str = "1/39";

fn main() {
    println!("Parsing dict until page {}...", NEXT_PAGE);

    let DictData(lines_noverbs, lines_verbs) = load_data(NEXT_PAGE).expect("Error loading data");

    let noverbs_total = lines_noverbs.len() as f32;
    let mut noverbs_success = 0.;

    println!("--------- NOVERBS ---------");

    for (index, line) in lines_noverbs.into_iter().enumerate() {
        // todo: parse skipped lines
        if line.contains(['|', '~', '=']) {
            continue;
        }

        let entry = general::parse(&line);

        match entry {
            Err(e) => {
                eprintln!("{index}: {line}");
                eprintln!("{:#?}\n", e);
            }
            Ok(entry) => {
                noverbs_success += 1.;
                println!("{index}: {line}");
                println!("{:?}\n", entry);
            }
        }
    }

    let noverbs_success_percent = noverbs_success / noverbs_total * 100.;
    println!(
        "Noverbs parsed successfully: {:.1}% ({}/{})",
        noverbs_success_percent, noverbs_success, noverbs_total
    );

    let verbs_total = lines_verbs.len() as f32;
    let mut verbs_success = 0.;

    println!("--------- VERBS ---------");

    for (index, line) in lines_verbs.into_iter().enumerate() {
        let entry = verb::parse(&line);

        match entry {
            Err(e) => {
                eprintln!("{index}: {line}");
                eprintln!("{:#?}\n", e);
            }
            Ok(entry) => {
                verbs_success += 1.;
                println!("{index}: {line}");
                println!("{:?}\n", entry);
            }
        }
    }

    let verbs_success_percent = verbs_success / verbs_total * 100.;
    println!(
        "Verbs parsed successfully: {:.1}% ({}/{})",
        verbs_success_percent, verbs_success, verbs_total
    );
}
