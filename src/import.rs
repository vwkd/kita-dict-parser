use regex::Regex;
use std::fs;
use std::io;

const PATH: &str = "../kita-dict-data/src/dict.txt";
const NEXT_PAGE: &str = "1/39";

#[derive(Debug)]
pub enum ImportError {
    Io(io::Error),
    Parse(regex::Error),
}

pub fn load_data() -> Result<String, ImportError> {
    let f = load_file(PATH).map_err(ImportError::Io)?;
    preprocess(&f).map_err(ImportError::Parse)
}

fn load_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn preprocess(text: &str) -> Result<String, regex::Error> {
    let re_next_page = Regex::new(&format!(r"\n\n## {}", NEXT_PAGE))?;
    let re_header_lines = Regex::new(r"(?m)^##.*\n")?;
    let re_empty_lines = Regex::new(r"(?m)^\n")?;
    let re_continued_lines = Regex::new(r"\n♦︎")?;

    let s0 = re_next_page.split(text).next().unwrap();
    let s1 = re_header_lines.replace_all(s0, "");
    let s2 = re_empty_lines.replace_all(&s1, "");
    let s3 = re_continued_lines.replace_all(&s2, "");
    
    Ok(s3.into_owned())
}