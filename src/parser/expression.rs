use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::error::ParseError;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

use super::character::ws_parser;
use super::field::{field_parser, Field};
use super::reference::{reference_parser, Reference};
use super::Index;

/*
Expression
    Usages
    Usage
*/
pub enum Expression<'a> {
    Usage(Usage<'a>),
    Usages(Usages<'a>),
}

pub fn expression_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, Expression, E> {
    alt((usage_parser, usages_parser))(input)
}

/*
Usages
    UsageItem(1) ws UsageItem(2) (ws UsageItem(i))_i=3*
*/
pub struct Usages<'a>(Vec<UsageItem<'a>>);

pub fn usages_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Usages, E> {
    // todo: create and use separated_list2
    separated_list1(ws_parser, usage_item_parser)(input)
}

/*
UsageItem(i)
    i "." ws Usage
*/
pub struct UsageItem<'a>(Usage<'a>, Index);

pub fn usage_item_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, UsageItem, E> {
    let (input, (i, u)) = separated_pair(
        map_res(digit1, |s: &str| s.parse::<u8>()),
        ws_parser,
        usage_parser,
    )(input)?;
    Ok((input, (u, i)))
}

// todo: part of speech (pos)
/*
Usage
    Definition (";" ws Definition)*
*/
pub struct Usage<'a>(Vec<DefinitionItem<'a>>);

pub fn usage_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Usage, E> {
    separated_list1(tag("; "), definition_parser)(input)
}

// todo: how to get definition item without state???
pub struct DefinitionItem<'a>(Definition<'a>, Index);

/*
Definition
    Reference
    Field
*/
pub enum Definition<'a> {
    Reference(Reference<'a>),
    Field(Field<'a>),
}

pub fn definition_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, Definition, E> {
    alt((reference_parser, field_parser))
}
