use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, value},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

use super::character::ws_parser;

/*
TagsWhitespace
    Tags ws
*/
pub fn tags_whitespace_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, Tags, E> {
    terminated(tags_parser, ws_parser)(input)
}

/*
Tags
    "{" Tag ("," ws Tag)* "}"
*/
#[derive(Debug)]
pub struct Tags(Vec<Tag>);

pub fn tags_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Tags, E> {
    map(
        delimited(char('{'), separated_list1(tag(", "), tag_parser), char('}')),
        Tags,
    )(input)
}

/*
Tag
    "biol."
    ...
*/
#[derive(Debug, Clone)]
pub enum Tag {
    BIOL,
    // ...
}

/* Rename tags
* remove trailing period and make uppercase
*/
pub fn tag_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Tag, E> {
    alt((
        value(Tag::BIOL, tag("biol.")),
        // ...
    ))(input)
}