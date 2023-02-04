use super::character::superscript_number_parser;
use super::word_ka::{headword_ka_parser, WordRootKa};
use nom::combinator::{map, opt};
use nom::sequence::tuple;
use nom::{error::ParseError, IResult};

use super::Index;

/*
Term
  HeadwordKa SuperscriptNumber?
*/
#[derive(Debug)]
pub struct Term<'a>(WordRootKa<'a>, Option<Index>);

pub fn term_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Term, E> {
    map(
        tuple((headword_ka_parser, opt(superscript_number_parser))),
        |(value, index)| Term(value, index),
    )(input)
}
