use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, map_res, opt};
use nom::error::{context, VerboseError};
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair, terminated, tuple};
use nom::IResult;

use crate::parser::utils::separated_list2;

use super::super::utils::KitaError;
use super::category::{categories_parser, Categories};
use super::character::{integer_parser, ws_parser};
use super::part_of_speech::{part_of_speech_tag_parser, PartOfSpeechTag};
use super::reference::{reference_parser, Reference};
use super::sentence_de::sentence_de_parser;
use super::symbols::{temporality_parser, Temporality};
use super::Index;

/*
Expression
    Usages
    UsageTagged
*/
#[derive(Debug)]
pub enum Expression<'a> {
    Usages(Usages<'a>),
    UsageTagged(UsageTagged<'a>),
}

pub fn expression_parser(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    context(
        "expression",
        alt((
            map(usages_parser, Expression::Usages),
            map(usage_tagged_parser, Expression::UsageTagged),
        )),
    )(input)
}

/*
Usages
    Tag UsageItem(1) ws UsageItem(2) (ws UsageItem(i))_i=3*
    UsageItemTagged(1) ws UsageItemTagged(2) (ws UsageItemTagged(i))_i=3*
*/
#[derive(Debug)]
pub enum Usages<'a> {
    Common(Tag, Vec<UsageItem<'a>>),
    Individual(Vec<UsageItemTagged<'a>>),
}

pub fn usages_parser(input: &str) -> IResult<&str, Usages, VerboseError<&str>> {
    context(
        "usages",
        alt((
            map_res(
                pair(tag_parser, separated_list2(ws_parser, usage_item_parser)),
                |(tag, usage_items)| {
                    // validate that integers are increasing with step 1
                    let is_increasing = usage_items
                        .iter()
                        .enumerate()
                        .all(|(i, val)| val.1 == i as u8 + 1);
                    if !is_increasing {
                        return Err(nom::Err::Error(KitaError::IncreasingUsagesList));
                    }
                    Ok(Usages::Common(tag, usage_items))
                },
            ),
            map_res(
                separated_list2(ws_parser, usage_item_tagged_parser),
                |usage_items_tagged| {
                    // validate that integers are increasing with step 1
                    let is_increasing = usage_items_tagged
                        .iter()
                        .enumerate()
                        .all(|(i, val)| val.1 == i as u8 + 1);
                    if !is_increasing {
                        return Err(nom::Err::Error(KitaError::IncreasingUsagesList));
                    }
                    Ok(Usages::Individual(usage_items_tagged))
                },
            ),
        )),
    )(input)
}

/*
UsageItemTagged(i)
    i "." ws UsageTagged
*/
#[derive(Debug)]
pub struct UsageItemTagged<'a>(UsageTagged<'a>, Index);

pub fn usage_item_tagged_parser(input: &str) -> IResult<&str, UsageItemTagged, VerboseError<&str>> {
    context(
        "usage_item_tagged",
        map(
            separated_pair(
                integer_parser,
                terminated(char('.'), ws_parser),
                usage_tagged_parser,
            ),
            |(index, usage_tagged)| UsageItemTagged(usage_tagged, index),
        ),
    )(input)
}

/*
UsageTagged
    Tag Usage
*/
#[derive(Debug)]
pub struct UsageTagged<'a>(Tag, Usage<'a>);

pub fn usage_tagged_parser(input: &str) -> IResult<&str, UsageTagged, VerboseError<&str>> {
    context(
        "usage_tagged",
        map(tuple((tag_parser, usage_parser)), |(tag, usage)| {
            UsageTagged(tag, usage)
        }),
    )(input)
}

/*
Tag
    (PartOfSpeechTag ws)? (Categories ws)? (Temporality ws)?
*/
#[derive(Debug)]
pub struct Tag(
    Option<PartOfSpeechTag>,
    Option<Categories>,
    Option<Temporality>,
);

pub fn tag_parser(input: &str) -> IResult<&str, Tag, VerboseError<&str>> {
    context(
        "tag",
        map(
            tuple((
                opt(terminated(part_of_speech_tag_parser, ws_parser)),
                opt(terminated(categories_parser, ws_parser)),
                opt(terminated(temporality_parser, ws_parser)),
            )),
            |(part_of_speech, categories, temporality)| {
                Tag(part_of_speech, categories, temporality)
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

pub fn usage_item_parser(input: &str) -> IResult<&str, UsageItem, VerboseError<&str>> {
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

/*
Usage
    Definition (";" ws Definition)*
*/
// todo: make DefinitionItem indirection, how to get index without state?
//pub struct DefinitionItem<'a>(Definition<'a>, Index);
#[derive(Debug)]
pub struct Usage<'a>(Vec<Definition<'a>>);

pub fn usage_parser(input: &str) -> IResult<&str, Usage, VerboseError<&str>> {
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

pub fn definition_parser(input: &str) -> IResult<&str, Definition, VerboseError<&str>> {
    context(
        "definition",
        alt((
            map(reference_parser, Definition::Reference),
            map(sentence_de_parser, Definition::SentenceDe),
        )),
    )(input)
}
