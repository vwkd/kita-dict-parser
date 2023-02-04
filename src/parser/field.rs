use nom::{error::ParseError, IResult, sequence::{tuple, pair, delimited}, multi::separated_list1, character::complete::char, bytes::complete::tag};

use super::{tag::{tags_whitespace_parser, Tags}, word::Words};
use super::word::words_parser;

use super::Value;

// TODO: Rename this module

/*
Field
    TagsWhitespace? Elements
*/
pub struct Field<'a>(Option<Tags>, Elements<'a>);

pub fn field_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Field, E> {
    tuple((tags_whitespace_parser, elements_parser))(input)
}

/*
Elements
    Element ("," ws Element)*
*/
pub struct Elements<'a>(Vec<Element<'a>>);

pub fn elements_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Elements, E> {
    separated_list1(char(','), element_parser)(input)
}

/*
Element
    Words Categories?
*/
pub struct Element<'a>(Value<'a>, Option<Categories<'a>>);

pub fn element_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Element, E> {
    pair(words_parser, categories_parser)(input)
}

/*
Categories
    ws "(" Words ("," ws Words)* ")"
*/
pub struct Categories<'a>(Vec<Words<'a>>);

pub fn categories_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Element, E> {
    delimited(tag(" ("), separated_list1(tag(", "), words_parser), tag(")"))(input)
}

