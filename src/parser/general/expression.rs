use std::num::ParseIntError;

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::error::{FromExternalError, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

use super::character::{integer_parser, ws_parser};
use super::reference::{reference_parser, Reference};
use super::word::sentence_de_parser;
use super::Index;

/*
Expression
    Usages
    Usage
*/
#[derive(Debug)]
pub enum Expression<'a> {
    Usage(Usage<'a>),
    Usages(Usages<'a>),
}

pub fn expression_parser<'i, E>(
    input: &'i str,
) -> IResult<&'i str, Expression, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    alt((
        map(usages_parser, Expression::Usages),
        map(usage_parser, Expression::Usage),
    ))(input)
}

/*
Usages
    UsageItem(1) ws UsageItem(2) (ws UsageItem(i))_i=3*
*/
// todo: implement increasing integers, probably needs custom parser
#[derive(Debug)]
pub struct Usages<'a>(Vec<UsageItem<'a>>);

pub fn usages_parser<'i, E>(input: &'i str) -> IResult<&'i str, Usages, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    // todo: create and use separated_list2
    map(separated_list1(ws_parser, usage_item_parser), Usages)(input)
}

/*
UsageItem(i)
    i "." ws Usage
*/
#[derive(Debug)]
pub struct UsageItem<'a>(Usage<'a>, Index);

pub fn usage_item_parser<'i, E>(input: &'i str) -> IResult<&'i str, UsageItem, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    map(
        separated_pair(
            integer_parser,
            terminated(char('.'), ws_parser),
            usage_parser,
        ),
        |(index, usage)| UsageItem(usage, index),
    )(input)
}

// todo: part of speech (pos)
/*
Usage
    Definition (";" ws Definition)*
*/
// todo: make DefinitionItem indirection, how to get index without state?
//pub struct DefinitionItem<'a>(Definition<'a>, Index);
#[derive(Debug)]
pub struct Usage<'a>(Vec<Definition<'a>>);

pub fn usage_parser<'i, E>(input: &'i str) -> IResult<&'i str, Usage, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    map(
        separated_list1(terminated(char(';'), ws_parser), definition_parser),
        Usage,
    )(input)
}

/*
Definition
    Reference
    SentenceDe
*/
#[derive(Debug)]
pub enum Definition<'a> {
    Reference(Reference<'a>),
    SentenceDe(&'a str),
}

pub fn definition_parser<'i, E>(input: &'i str) -> IResult<&'i str, Definition, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    alt((
        map(reference_parser, Definition::Reference),
        map(sentence_de_parser, Definition::SentenceDe),
    ))(input)
}
