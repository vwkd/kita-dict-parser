mod category;
mod character;
mod conjugation;
mod expression;
mod form;
mod term;

use winnow::combinator::separated_pair;
use winnow::prelude::*;
use winnow::{combinator::alt, combinator::repeat, error::StrContext};

use character::nlwsws_parser;
use form::{form_parser, VerbSingleForm};
use term::{term_infinitive_parser, term_parser, VerbTerm, VerbTermInfinitive};

/*
Parser
  VerbSingleEntry EOF
  VerbMultiEntry EOF
*/
#[derive(Debug)]
pub enum VerbEntry<'a> {
    Single(VerbSingleEntry<'a>),
    Multi(VerbMultiEntry<'a>),
}

pub fn parse(input: &mut str) -> Result<VerbEntry, String> {
    alt((
        single_entry_parser.map(VerbEntry::Single),
        multi_entry_parser.map(VerbEntry::Multi),
    ))
    .parse(input)
    .map_err(|e| e.to_string())
}

/*
VerbSingleEntry
  VerbTermInfinitive nlwsws VerbSingleForm
*/
#[derive(Debug)]
pub struct VerbSingleEntry<'a>(VerbTermInfinitive<'a>, VerbSingleForm<'a>);

pub fn single_entry_parser<'a>(input: &mut &'a str) -> PResult<VerbSingleEntry<'a>> {
    separated_pair(term_infinitive_parser, nlwsws_parser, form_parser)
        .map(|(term, form)| VerbSingleEntry(term, form))
        .context(StrContext::Label("single_entry"))
        .parse_next(input)
}

/*
VerbMultiEntry
  VerbTerm nlwsws VerbForm+
*/
#[derive(Debug)]
pub struct VerbMultiEntry<'a>(VerbTerm<'a>, Vec<VerbForm<'a>>);

pub fn multi_entry_parser<'a>(input: &mut &'a str) -> PResult<VerbMultiEntry<'a>> {
    separated_pair(term_parser, nlwsws_parser, repeat(1.., entry_parser))
        .map(|(term, forms)| VerbMultiEntry(term, forms))
        .context(StrContext::Label("multi_entry"))
        .parse_next(input)
}

/*
VerbForm
  // ...
*/
#[derive(Debug)]
pub struct VerbForm<'a>(&'a str); // todo:

pub fn entry_parser<'a>(input: &mut &'a str) -> PResult<VerbForm<'a>> {
    todo!()
}
