use super::category::{categories_parser, Categories};
use super::character::{integer_parser, ws_parser};
use super::term::{term_parser, Term};
use super::Index;
use nom::combinator::{map, opt};
use nom::error::{context, VerboseError};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value,
    sequence::preceded, IResult,
};

/*
Reference
    (Categories ws)? ReferenceKind ws Term WhitespaceUsageIndex?
*/
#[derive(Debug)]
pub struct Reference<'a>(Term<'a>, Option<Index>, ReferenceKind, Option<Categories>);

pub fn reference_parser(input: &str) -> IResult<&str, Reference, VerboseError<&str>> {
    context(
        "reference",
        map(
            separated_pair(
                tuple((
                    opt(terminated(categories_parser, ws_parser)),
                    reference_kind_parser,
                )),
                ws_parser,
                tuple((term_parser, opt(whitespace_usage_index_parser))),
            ),
            |((categories, kind), (term, index))| Reference(term, index, kind, categories),
        ),
    )(input)
}

/*
WhitespaceUsageIndex
    ws "(" "Pkt." ws Integer ")"
*/
pub fn whitespace_usage_index_parser(input: &str) -> IResult<&str, u8, VerboseError<&str>> {
    context(
        "whitespace_usage_index",
        preceded(
            ws_parser,
            delimited(
                char('('),
                preceded(terminated(tag("Pkt."), ws_parser), integer_parser),
                char(')'),
            ),
        ),
    )(input)
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

pub fn reference_kind_parser(input: &str) -> IResult<&str, ReferenceKind, VerboseError<&str>> {
    context(
        "reference_kind",
        alt((
            value(
                ReferenceKind::SeeMeaning,
                separated_pair(tag("Bed."), ws_parser, tag("s.")),
            ),
            value(ReferenceKind::See, tag("s.")),
        )),
    )(input)
}
