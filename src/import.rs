use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

const PATH: &str = "../kita-dict-data/src/dict.txt";

#[derive(Debug)]
pub enum ImportError {
    Io(io::Error),
    Parse(regex::Error),
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImportError::Io(err) => write!(f, "IO error: {}", err),
            ImportError::Parse(err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl Error for ImportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ImportError::Io(err) => Some(err),
            ImportError::Parse(err) => Some(err),
        }
    }
}

impl From<io::Error> for ImportError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<regex::Error> for ImportError {
    fn from(err: regex::Error) -> Self {
        Self::Parse(err)
    }
}

/// first field is collection of all entries except verbs,
/// second is collection of verb entries
pub struct DictData(pub Vec<String>, pub Vec<String>);

pub fn load_data(next_page: &str) -> Result<DictData, ImportError> {
    let text = load_file(PATH)?;
    let data = preprocess(&text, next_page)?;
    Ok(data)
}

fn load_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Returns a tuple with the dict text without verb lines and the dict text with verb lines
fn preprocess(text: &str, next_page: &str) -> Result<DictData, regex::Error> {
    let re_next_page = Regex::new(&format!(r"\n\n## {}", next_page))?;
    let text0 = re_next_page
        .split(text)
        .next()
        .expect(&format!("Page '{}' not found", next_page));

    let re_header_lines = Regex::new(r"(?m)^##.*\n")?;
    let text1 = re_header_lines.replace_all(text0, "");

    let re_empty_lines = Regex::new(r"(?m)^\n")?;
    let text2 = re_empty_lines.replace_all(&text1, "");

    let re_continued_lines = Regex::new(r"\n♦︎")?;
    let text = re_continued_lines.replace_all(&text2, "");

    let re_verb_lines = Regex::new(r"(?m)^.*\n(^  .*\n)+")?;

    // let text_noverbs = re_verb_lines.replace_all(&text, "").to_string();
    let text_noverbs = re_verb_lines.split(&text).join("");
    let lines_noverbs: Vec<String> = text_noverbs.lines().map(|l| l.to_owned()).collect();

    // trim empty last line
    let lines_verbs: Vec<String> = re_verb_lines
        .find_iter(&text)
        .map(|m| m.as_str().trim_end().to_owned())
        .collect();

    Ok(DictData(lines_noverbs, lines_verbs))
}
