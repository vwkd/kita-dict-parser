use super::character::ws_parser;
use super::tag::{tags_whitespace_parser, Tags};
use super::term::{term_parser, Term};
use super::Index;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::{branch::alt, bytes::complete::tag, combinator::value, error::ParseError, IResult};

/*
Reference
    TagsWhitespace? ReferenceKind ws Term WhitespaceUsageIndex?
*/
pub struct Reference<'a>(Term<'a>, Option<Index>, ReferenceKind, Option<Tags>);

pub fn reference_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, Reference, E> {
    separated_pair(
        tuple((tags_whitespace_parser, reference_kind_parser)),
        ws_parser,
        tuple((term_parser, whitespace_usage_index_parser)),
    )
}

/*
WhitespaceUsageIndex
    ws "(Pkt." ws Digit ")"
*/
pub fn whitespace_usage_index_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, u8, E> {
    map_res(delimited(tag(" (Pkt. "), digit1, tag(")")), |s: &str| {
        s.parse::<u8>()
    })
}

/*
ReferenceKind
    "Bed." ws "s."
    "s."
*/
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
    ))
}
