use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, map_res};
use nom::error::context;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use nom_supreme::error::ErrorTree;

use super::super::utils::KitaError;
use super::character::{integer_parser, ws_parser};
use super::reference::{reference_parser, Reference};
use super::sentence_de::sentence_de_parser;
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
    context(
        "expression",
        alt((
            map(usages_parser, Expression::Usages),
            map(usage_parser, Expression::Usage),
        )),
    )(input)
}

/*
Usages
    UsageItem(1) ws UsageItem(2) (ws UsageItem(i))_i=3*
*/
#[derive(Debug)]
pub struct Usages<'a>(Vec<UsageItem<'a>>);

pub fn usages_parser(input: &str) -> IResult<&str, Usages, ErrorTree<&str>> {
    context(
        "usages",
        map_res(
            separated_pair(
                usage_item_parser,
                ws_parser,
                separated_list1(ws_parser, usage_item_parser),
            ),
            |(first, mut rest)| {
                // hack for missing `separated_list2`
                rest.insert(0, first);
                // validate that integers are increasing with step 1
                let is_increasing = rest.iter().enumerate().all(|(i, val)| val.1 == i as u8 + 1);
                if !is_increasing {
                    return Err(nom::Err::Error(KitaError::IncreasingUsagesList));
                }
                Ok(Usages(rest))
            },
        ),
    )(input)
}

/*
UsageItem(i)
    i "." ws Usage
*/
#[derive(Debug)]
pub struct UsageItem<'a>(Usage<'a>, Index);

pub fn usage_item_parser(input: &str) -> IResult<&str, UsageItem, ErrorTree<&str>> {
    context(
        "usage_item",
        map(
            separated_pair(
                integer_parser,
                terminated(char('.'), ws_parser),
                usage_parser,
            ),
            |(index, usage)| UsageItem(usage, index),
        ),
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
    context(
        "usage",
        map(
            separated_list1(terminated(char(';'), ws_parser), definition_parser),
            Usage,
        ),
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
    context(
        "definition",
        alt((
            map(reference_parser, Definition::Reference),
            map(sentence_de_parser, Definition::SentenceDe),
        )),
    )(input)
}
