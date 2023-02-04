mod character;
mod expression;
mod field;
mod reference;
mod tag;
mod term;
mod word;
mod word_de;
mod word_ka;

use character::ws_parser;
use expression::{expression_parser, Expression};
use nom::{combinator::eof, error::ParseError, sequence::separated_pair, IResult};
use term::{term_parser, Term};

pub type Value<'a> = &'a str;
pub type Index = u8;

/*
Parser
  Entry EOF
*/
pub fn parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Entry, E> {
    eof(entry_parser(input))
}

/*
Entry
  Term ws Expression
*/
pub struct Entry<'a>(Term<'a>, Expression<'a>);

pub fn entry_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Entry, E> {
    separated_pair(term_parser, ws_parser, expression_parser)(input)
}
