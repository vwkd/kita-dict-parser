use regex::Regex;
use std::fs;
use std::io;

const PATH: &str = "../kita-dict-data/src/dict.txt";

#[derive(Debug)]
pub enum ImportError {
    Io(io::Error),
    Parse(regex::Error),
}

pub fn load_data(next_page: &str) -> Result<String, ImportError> {
    let f = load_file(PATH).map_err(ImportError::Io)?;
    preprocess(&f, next_page).map_err(ImportError::Parse)
}

fn load_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn preprocess(text: &str, next_page: &str) -> Result<String, regex::Error> {
    let re_next_page = Regex::new(&format!(r"\n\n## {}", next_page))?;
    let s0 = re_next_page
        .split(text)
        .next()
        .expect(&format!("Page '{}' not found", next_page));

    let re_header_lines = Regex::new(r"(?m)^##.*\n")?;
    let s1 = re_header_lines.replace_all(s0, "");

    let re_empty_lines = Regex::new(r"(?m)^\n")?;
    let s2 = re_empty_lines.replace_all(&s1, "");

    let re_continued_lines = Regex::new(r"\n♦︎")?;
    let s3 = re_continued_lines.replace_all(&s2, "");

    // todo: keep verb lines and parse
    let re_verb_lines = Regex::new(r"(?m)^.*\n(^  .*\n)+")?;
    let s4 = re_verb_lines.replace_all(&s3, "");

    Ok(s4.into_owned())
}
