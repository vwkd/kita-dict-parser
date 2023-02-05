pub mod character;
pub mod expression;
pub mod field;
pub mod reference;
pub mod tag;
pub mod term;
pub mod word;
pub mod word_de;
pub mod word_ka;

use std::num::ParseIntError;

use character::ws_parser;
use expression::{expression_parser, Expression};
use term::{term_parser, Term};
use nom::{
    combinator::{eof, map},
    error::{FromExternalError, ParseError},
    sequence::{separated_pair, terminated},
    IResult,
};

pub type Value<'a> = &'a str;
pub type Index = u8;

/*
Parser
  Entry EOF
*/
pub fn parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Entry, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    terminated(entry_parser, eof)(input)
}

/*
Entry
  Term ws Expression
*/
#[derive(Debug)]
pub struct Entry<'a>(Term<'a>, Expression<'a>);

pub fn entry_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Entry<'i>, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    map(
        separated_pair(term_parser, ws_parser, expression_parser),
        |(t, e)| Entry(t, e),
    )(input)
}
