use winnow::combinator::opt;
use winnow::error::StrContext;
use winnow::prelude::*;

use super::character::superscript_number_parser;
use super::sentence_ka::{headword_ka_parser, HeadwordKa};
use super::Index;

/*
Term
  HeadwordKa SuperscriptNumber?
*/
#[derive(Debug)]
pub struct Term<'a>(HeadwordKa<'a>, Option<Index>);

pub fn term_parser<'a>(input: &mut &'a str) -> PResult<Term<'a>> {
    (headword_ka_parser, opt(superscript_number_parser))
        .map(|(value, index)| Term(value, index))
        .context(StrContext::Label("term"))
        .parse_next(input)
}
