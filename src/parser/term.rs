use super::character::superscript_number_parser;
use super::word_ka::word_ka_parser;
use nom::combinator::{opt, map};
use nom::sequence::tuple;
use nom::{error::ParseError, IResult};

use super::{Index, Value};

/*
Term
  WordKa SuperscriptNumber?
*/
#[derive(Debug)]
pub struct Term<'a>(Value<'a>, Option<Index>);

pub fn term_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, Term, E> {
    map(tuple((word_ka_parser, opt(superscript_number_parser))), |(value, index)| Term(value, index))(input)
}
