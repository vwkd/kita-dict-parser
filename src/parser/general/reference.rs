use std::num::ParseIntError;

use super::character::ws_parser;
use super::tag::{tags_whitespace_parser, Tags};
use super::term::{term_parser, Term};
use super::Index;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, opt};
use nom::error::FromExternalError;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::{branch::alt, bytes::complete::tag, combinator::value, error::ParseError, IResult};

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
    ws "(Pkt." ws Digit ")"
*/
pub fn whitespace_usage_index_parser<'i, E>(input: &'i str) -> IResult<&'i str, u8, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    delimited(
        tag(" (Pkt. "),
        map_res(digit1, |s: &str| s.parse::<u8>()),
        tag(")"),
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
        value(ReferenceKind::SeeMeaning, tag("Bed. s.")),
        value(ReferenceKind::See, tag("s.")),
    ))(input)
}
