use std::num::ParseIntError;

use nom::{
    combinator::map,
    error::{FromExternalError, ParseError},
    IResult,
};

use crate::parser::general::word::sentence_de_parser;

/*
VerbExpression
  WordsDe
*/
// todo: add multiple
#[derive(Debug)]
pub struct VerbExpression<'a>(&'a str);

pub fn expression_parser<'i, E>(input: &'i str) -> IResult<&'i str, VerbExpression<'i>, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    map(sentence_de_parser, VerbExpression)(input)
}
