use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, value},
    error::context,
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};
use nom_supreme::error::ErrorTree;

use super::character::ws_parser;

/*
TagsWhitespace
    Tags ws
*/
pub fn tags_whitespace_parser(input: &str) -> IResult<&str, Tags, ErrorTree<&str>> {
    context("tags_whitespace", terminated(tags_parser, ws_parser))(input)
}

/*
Tags
    "{" Tag ("," ws Tag)* "}"
*/
#[derive(Debug)]
pub struct Tags(Vec<Tag>);

pub fn tags_parser(input: &str) -> IResult<&str, Tags, ErrorTree<&str>> {
    context(
        "tags",
        map(
            delimited(
                char('{'),
                separated_list1(terminated(char(','), ws_parser), tag_parser),
                char('}'),
            ),
            Tags,
        ),
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
pub fn tag_parser(input: &str) -> IResult<&str, Tag, ErrorTree<&str>> {
    context(
        "tag",
        alt((
            value(Tag::BIOL, tag("biol.")),
            // ...
        )),
    )(input)
}
