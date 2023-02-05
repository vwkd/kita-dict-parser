use nom::{combinator::map, error::ParseError, sequence::tuple, IResult};

use crate::parser::general::character::ws_parser;

use super::{
    category::{category_parser, VerbCategory},
    conjugation::{conjugation_parser, VerbConjugation},
    expression::{expression_parser, VerbExpression},
};

/*
VerbForm
  VerbCategory ws VerbConjugation ws VerbExpression
*/
#[derive(Debug)]
pub struct VerbForm<'a>(VerbCategory, VerbConjugation<'a>, VerbExpression<'a>);

pub fn form_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, VerbForm<'i>, E> {
    map(
        tuple((
            category_parser,
            ws_parser,
            conjugation_parser,
            ws_parser,
            expression_parser,
        )),
        |(category, _, conjugation, _, expression)| {
            VerbForm(category, conjugation, expression)
        },
    )(input)
}
