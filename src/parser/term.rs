use std::num::ParseIntError;

use super::character::superscript_number_parser;
use super::word_ka::word_ka_parser;
use nom::combinator::opt;
use nom::error::FromExternalError;
use nom::sequence::tuple;
use nom::{error::ParseError, IResult};

use super::{Index, Value};

/*
Term
  WordKa SuperscriptNumber?
*/
pub struct Term<'a>(Value<'a>, Option<Index>);

pub fn term_parser<'i, E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>>(
    input: &'i str,
) -> IResult<&'i str, Term, E> {
    tuple((word_ka_parser, opt(superscript_number_parser)))(input)
}
