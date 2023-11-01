use winnow::combinator::{alt, delimited, opt, preceded, separated_pair, terminated};
use winnow::error::StrContext;
use winnow::prelude::*;

use super::category::{categories_parser, Categories};
use super::character::{integer_parser, ws_parser};
use super::term::{term_parser, Term};
use super::Index;

/*
Reference
    (Categories ws)? ReferenceKind ws Term WhitespaceUsageIndex?
*/
#[derive(Debug)]
pub struct Reference<'a>(Term<'a>, Option<Index>, ReferenceKind, Option<Categories>);

pub fn reference_parser<'a>(input: &mut &'a str) -> PResult<Reference<'a>> {
    separated_pair(
        (
            opt(terminated(categories_parser, ws_parser)),
            reference_kind_parser,
        ),
        ws_parser,
        (term_parser, opt(whitespace_usage_index_parser)),
    )
    .map(|((categories, kind), (term, index))| Reference(term, index, kind, categories))
    .context(StrContext::Label("reference"))
    .parse_next(input)
}

/*
WhitespaceUsageIndex
    ws "(" "Pkt." ws Integer ")"
*/
pub fn whitespace_usage_index_parser<'a>(input: &mut &'a str) -> PResult<u8> {
    preceded(
        ws_parser,
        delimited(
            '(',
            preceded(terminated("Pkt.", ws_parser), integer_parser),
            ')',
        ),
    )
    .context(StrContext::Label("whitespace_usage_index"))
    .parse_next(input)
}

/*
ReferenceKind
    "Bed." ws "s."
    "s."
*/
#[derive(Debug, Clone)]
pub enum ReferenceKind {
    SeeMeaning,
    See,
}

pub fn reference_kind_parser<'a>(input: &mut &'a str) -> PResult<ReferenceKind> {
    alt((
        separated_pair("Bed.", ws_parser, "s.").value(ReferenceKind::SeeMeaning),
        "s.".value(ReferenceKind::See),
    ))
    .context(StrContext::Label("reference_kind"))
    .parse_next(input)
}
