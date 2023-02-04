use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::value,
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
pub struct Tags(Vec<Tag>);

pub fn tags_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Tags, E> {
    delimited(char('{'), separated_list1(tag(", "), tag_parser), char('}'))(input)
}

/*
Tag
    "biol."
    ...
*/
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
