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
pub enum ImportError<'a> {
    #[error("can't load file")]
    LoadingFile(#[from] io::Error),
    #[error("can't find page '{0}'")]
    PageNotFound(&'a str),
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
pub fn get_entries<'a>(text: &str, next_page: &'a str) -> Result<Dict, ImportError<'a>> {
    let next_page_header = format!("\n\n## {next_page}");
    let (text, _) = text
        .split_once(&next_page_header)
        .ok_or_else(|| ImportError::PageNotFound(next_page))?;

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
