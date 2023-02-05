mod category;
mod character;
mod conjugation;
mod expression;
mod form;
mod term;

use nom::{
    combinator::{eof, map},
    error::ParseError,
    sequence::{separated_pair, terminated},
    IResult,
};

use form::{form_parser, VerbForm};
use term::{term_parser, VerbTerm};
use character::nltb_parser;

/*
Parser
  VerbEntry EOF
*/
pub fn parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, VerbEntry, E> {
    terminated(entry_parser, eof)(input)
}

/*
VerbEntry
  VerbTerm nltb VerbForm
*/
#[derive(Debug)]
pub struct VerbEntry<'a>(VerbTerm<'a>, VerbForm<'a>);

pub fn entry_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, VerbEntry, E> {
    map(separated_pair(term_parser, nltb_parser, form_parser), |(term, form)| {
        VerbEntry(term, form)
    })(input)
}
