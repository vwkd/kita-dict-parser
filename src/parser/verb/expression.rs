use nom::{
    combinator::map,
    error::{context, VerboseError},
    IResult,
};

use crate::parser::general::sentence_de::sentence_de_parser;

/*
VerbExpression
  WordsDe
*/
// todo: add multiple
#[derive(Debug)]
pub struct VerbExpression<'a>(&'a str);

pub fn expression_parser(input: &str) -> IResult<&str, VerbExpression, VerboseError<&str>> {
    context("expression", map(sentence_de_parser, VerbExpression))(input)
}
