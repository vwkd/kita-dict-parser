use nom::{
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

use super::tag::{tags_whitespace_parser, Tags};
use super::word::words_parser;

use super::Value;

// TODO: Rename this module

/*
Field
    TagsWhitespace? Elements
*/
#[derive(Debug)]
pub struct Field<'a>(Option<Tags>, Elements<'a>);

pub fn field_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Field, E> {
    map(
        tuple((opt(tags_whitespace_parser), elements_parser)),
        |(tags, elements)| Field(tags, elements),
    )(input)
}

/*
Elements
    Element ("," ws Element)*
*/
#[derive(Debug)]
pub struct Elements<'a>(Vec<Element<'a>>);

pub fn elements_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, Elements, E> {
    map(separated_list1(char(','), element_parser), Elements)(input)
}

/*
Element
    Words Categories?
*/
#[derive(Debug)]
pub struct Element<'a>(Value<'a>, Option<Categories<'a>>);

pub fn element_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Element, E> {
    map(
        pair(words_parser, opt(categories_parser)),
        |(value, categories)| Element(value, categories),
    )(input)
}

/*
Categories
    ws "(" Words ("," ws Words)* ")"
*/
#[derive(Debug)]
pub struct Categories<'a>(Vec<Value<'a>>);

pub fn categories_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, Categories, E> {
    map(
        delimited(
            tag(" ("),
            separated_list1(tag(", "), words_parser),
            tag(")"),
        ),
        Categories,
    )(input)
}
