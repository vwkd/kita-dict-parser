use nom::{combinator::map, error::ParseError, IResult};

use crate::parser::general::word::sentence_de_parser;

/*
VerbExpression
  WordsDe
*/
// todo: add multiple
#[derive(Debug)]
pub struct VerbExpression<'a>(&'a str);

pub fn expression_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, VerbExpression<'i>, E> {
    map(sentence_de_parser, VerbExpression)(input)
}
