pub mod character;
pub mod expression;
pub mod reference;
pub mod tag;
pub mod term;
pub mod word;
pub mod word_de;
pub mod word_ka;

use character::ws_parser;
use expression::{expression_parser, Expression};
use nom::{combinator::map, sequence::separated_pair, IResult};
use nom_supreme::{error::ErrorTree, final_parser::final_parser};
use term::{term_parser, Term};

pub type Value<'a> = &'a str;
pub type Index = u8;

/*
Parser
  Entry EOF
*/
pub fn parse(input: &str) -> Result<Entry, ErrorTree<&str>> {
    final_parser(entry_parser)(input)
}

/*
Entry
  Term ws Expression
*/
#[derive(Debug)]
pub struct Entry<'a>(Term<'a>, Expression<'a>);

pub fn entry_parser(input: &str) -> IResult<&str, Entry, ErrorTree<&str>> {
    map(
        separated_pair(term_parser, ws_parser, expression_parser),
        |(t, e)| Entry(t, e),
    )(input)
}
