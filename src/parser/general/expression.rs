use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use nom_supreme::error::ErrorTree;

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

pub fn expression_parser(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
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

pub fn usages_parser(input: &str) -> IResult<&str, Usages, ErrorTree<&str>> {
    // todo: create and use separated_list2
    map(separated_list1(ws_parser, usage_item_parser), Usages)(input)
}

/*
UsageItem(i)
    i "." ws Usage
*/
#[derive(Debug)]
pub struct UsageItem<'a>(Usage<'a>, Index);

pub fn usage_item_parser(input: &str) -> IResult<&str, UsageItem, ErrorTree<&str>> {
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

pub fn usage_parser(input: &str) -> IResult<&str, Usage, ErrorTree<&str>> {
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

pub fn definition_parser(input: &str) -> IResult<&str, Definition, ErrorTree<&str>> {
    alt((
        map(reference_parser, Definition::Reference),
        map(sentence_de_parser, Definition::SentenceDe),
    ))(input)
}
