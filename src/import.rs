use regex::Regex;
use std::{fs, io};
use thiserror::Error;

const DICT_FILEPATH: &str = "../kita-dict-data/src/dict.txt";

#[derive(Debug)]
pub enum Entry {
    Verb(String),
    Other(String),
}

pub type Dict = Vec<Entry>;

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("can't load file")]
    LoadingFile(#[from] io::Error),
    #[error("can't find page '{0}'")]
    PageNotFound(String),
    // #[error("can't get entries")]
    // GettingEntries,
}

pub fn load_dict(next_page: &str) -> Result<Dict, ImportError> {
    let text = fs::read_to_string(DICT_FILEPATH)?;
    get_entries(&text, next_page)
}

/// Get entries from dict files
/// - split page
/// - filter out header lines
/// - filter out empty lines
/// - merge page breaks
/// - partition into entries
pub fn get_entries(text: &str, next_page: &str) -> Result<Dict, ImportError> {
    let re_next_page = Regex::new(&format!(r"\n\n## {}", next_page)).expect("Invalid Regex");
    let text = re_next_page
        .split(text)
        .next()
        .ok_or_else(|| ImportError::PageNotFound(next_page.to_owned()))?;

    // todo: find way to use slices to avoid allocations
    let re_header_lines = Regex::new(r"(?m)^##.*\n").expect("Invalid Regex");
    let text = re_header_lines.replace_all(text, "");

    let re_empty_lines = Regex::new(r"(?m)^\n").expect("Invalid Regex");
    let text = re_empty_lines.replace_all(&text, "");

    let re_continued_lines = Regex::new(r"\n♦︎").expect("Invalid Regex");
    let text = re_continued_lines.replace_all(&text, "");

    let re_entry = Regex::new(r"(?m)^\S.+(?:\n  .+)*$").expect("Invalid Regex");

    // todo: make sure matching instead of splitting actually covers the whole string, i.e. no unmatched parts due to errors in the source text
    Ok(re_entry
        .find_iter(&text)
        .map(|entry| {
            if entry.as_str().contains('\n') {
                Entry::Verb(entry.as_str().to_owned())
            } else {
                Entry::Other(entry.as_str().to_owned())
            }
        })
        .collect())
}
