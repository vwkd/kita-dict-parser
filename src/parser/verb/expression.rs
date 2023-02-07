use nom::{combinator::map, error::context, IResult};
use nom_supreme::error::ErrorTree;

use crate::parser::general::word::sentence_de_parser;

/*
VerbExpression
  WordsDe
*/
// todo: add multiple
#[derive(Debug)]
pub struct VerbExpression<'a>(&'a str);

pub fn expression_parser(input: &str) -> IResult<&str, VerbExpression, ErrorTree<&str>> {
    context("expression", map(sentence_de_parser, VerbExpression))(input)
}
