mod category;
mod character;
mod conjugation;
mod expression;
mod form;
mod term;

use nom::{branch::alt, combinator::map, multi::many1, sequence::separated_pair, IResult};

use character::nlwsws_parser;
use form::{form_parser, VerbSingleForm};
use nom_supreme::{error::ErrorTree, final_parser::final_parser};
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

pub fn parse(input: &str) -> Result<VerbEntry, ErrorTree<&str>> {
    final_parser(alt((
        map(single_entry_parser, VerbEntry::Single),
        map(multi_entry_parser, VerbEntry::Multi),
    )))(input)
}

/*
VerbSingleEntry
  VerbTermInfinitive nlwsws VerbSingleForm
*/
#[derive(Debug)]
pub struct VerbSingleEntry<'a>(VerbTermInfinitive<'a>, VerbSingleForm<'a>);

pub fn single_entry_parser(input: &str) -> IResult<&str, VerbSingleEntry, ErrorTree<&str>> {
    map(
        separated_pair(term_infinitive_parser, nlwsws_parser, form_parser),
        |(term, form)| VerbSingleEntry(term, form),
    )(input)
}

/*
VerbMultiEntry
  VerbTerm nlwsws VerbForm+
*/
#[derive(Debug)]
pub struct VerbMultiEntry<'a>(VerbTerm<'a>, Vec<VerbForm<'a>>);

pub fn multi_entry_parser(input: &str) -> IResult<&str, VerbMultiEntry, ErrorTree<&str>> {
    map(
        separated_pair(term_parser, nlwsws_parser, many1(entry_parser)),
        |(term, forms)| VerbMultiEntry(term, forms),
    )(input)
}

/*
VerbForm
  // ...
*/
#[derive(Debug)]
pub struct VerbForm<'a>(&'a str); // todo:

pub fn entry_parser(input: &str) -> IResult<&str, VerbForm, ErrorTree<&str>> {
    todo!()
}
