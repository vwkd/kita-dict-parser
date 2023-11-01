use winnow::error::StrContext;
use winnow::prelude::*;

use crate::parser::general::sentence_de::sentence_de_parser;

/*
VerbExpression
  WordsDe
*/
// todo: add multiple
#[derive(Debug)]
pub struct VerbExpression<'a>(&'a str);

pub fn expression_parser<'a>(input: &mut &'a str) -> PResult<VerbExpression<'a>> {
    sentence_de_parser
        .map(VerbExpression)
        .context(StrContext::Label("expression"))
        .parse_next(input)
}
