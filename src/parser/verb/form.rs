use nom::{combinator::map, sequence::tuple, IResult};
use nom_supreme::error::ErrorTree;

use crate::parser::general::character::ws_parser;

use super::{
    category::{category_parser, VerbCategory},
    conjugation::{conjugation_parser, VerbConjugation},
    expression::{expression_parser, VerbExpression},
};

/*
VerbSingleForm
  VerbCategory ws VerbConjugation ws VerbExpression
*/
#[derive(Debug)]
pub struct VerbSingleForm<'a>(VerbCategory, VerbConjugation<'a>, VerbExpression<'a>);

pub fn form_parser(input: &str) -> IResult<&str, VerbSingleForm, ErrorTree<&str>> {
    map(
        tuple((
            category_parser,
            ws_parser,
            conjugation_parser,
            ws_parser,
            expression_parser,
        )),
        |(category, _, conjugation, _, expression)| {
            VerbSingleForm(category, conjugation, expression)
        },
    )(input)
}
