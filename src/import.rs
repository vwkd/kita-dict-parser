use std::{fs, io};
use thiserror::Error;

const DICT_FILEPATH: &str = "../kita-dict-data/src/dict.txt";

#[derive(Debug)]
pub enum Entry<'a> {
    Verb(Vec<&'a str>),
    Other(&'a str),
}

pub type Dict<'a> = Vec<Entry<'a>>;

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("can't load file")]
    LoadingFile(#[from] io::Error),
    #[error("can't find page '{0}'")]
    PageNotFound(String),
    // #[error("can't get entries")]
    // GettingEntries,
}

pub fn load_dict<'a>(next_page: &'a str) -> Result<Dict<'a>, ImportError> {
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
    let next_page_header = format!("\n\n## {next_page}");
    let (text, _) = text
        .split_once(&next_page_header)
        .ok_or_else(|| ImportError::PageNotFound(next_page.to_owned()))?;

    let lines: Vec<&str> = text
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with("##"))
        // .fold(Vec::new(), |mut acc, line| {
        //     acc.push(line);
        //     acc
        // });
        .collect();

    let lines: Vec<&str> = lines
        .group_by(|a, b| b.starts_with("♦︎"))
        .map(|x| {
            if x.len() == 2 && x[1].starts_with("♦︎") {
                let first = x[0];
                let second = x[1].strip_prefix("♦︎").unwrap();
                let joined = [first, second].concat();
                &[joined.as_str()]
            } else {
                x
            }
        })
        .flat_map(|&slice| slice)
        .collect();

    let lines: Dict = lines
        .group_by(|a, b| b.starts_with("  "))
        .map(|x| {
            if x.len() == 1 {
                let other = x[0];
                Entry::Other(other)
            } else {
                let clean: Vec<&str> = x
                    .iter()
                    .map(|l| l.trim_start_matches(' '))
                    .collect();
                Entry::Verb(clean)
            }
        })
        .collect();

    Ok(lines)
}
