use std::num::ParseIntError;

use super::character::{integer_parser, ws_parser};
use super::tag::{tags_whitespace_parser, Tags};
use super::term::{term_parser, Term};
use super::Index;
use nom::combinator::{map, opt};
use nom::error::FromExternalError;
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value,
    error::ParseError, sequence::preceded, IResult,
};

/*
Reference
    TagsWhitespace? ReferenceKind ws Term WhitespaceUsageIndex?
*/
#[derive(Debug)]
pub struct Reference<'a>(Term<'a>, Option<Index>, ReferenceKind, Option<Tags>);

pub fn reference_parser<'i, E>(input: &'i str) -> IResult<&'i str, Reference, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    map(
        separated_pair(
            tuple((opt(tags_whitespace_parser), reference_kind_parser)),
            ws_parser,
            tuple((term_parser, opt(whitespace_usage_index_parser))),
        ),
        |((tags, kind), (term, index))| Reference(term, index, kind, tags),
    )(input)
}

/*
WhitespaceUsageIndex
    ws "(" "Pkt." ws Integer ")"
*/
pub fn whitespace_usage_index_parser<'i, E>(input: &'i str) -> IResult<&'i str, u8, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    preceded(
        ws_parser,
        delimited(
            char('('),
            preceded(terminated(tag("Pkt."), ws_parser), integer_parser),
            char(')'),
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

pub fn reference_kind_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, ReferenceKind, E> {
    alt((
        value(
            ReferenceKind::SeeMeaning,
            separated_pair(tag("Bed."), ws_parser, tag("s.")),
        ),
        value(ReferenceKind::See, tag("s.")),
    ))(input)
}
