use winnow::error::StrContext;
use winnow::prelude::*;

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

pub fn form_parser<'a>(input: &mut &'a str) -> PResult<VerbSingleForm<'a>> {
    (
        category_parser,
        ws_parser,
        conjugation_parser,
        ws_parser,
        expression_parser,
    )
        .map(|(category, _, conjugation, _, expression)| {
            VerbSingleForm(category, conjugation, expression)
        })
        .context(StrContext::Label("form"))
        .parse_next(input)
}
