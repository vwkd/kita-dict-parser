use thiserror::Error;
use winnow::combinator::{alt, opt, separated, separated_pair, terminated};
use winnow::error::StrContext;
use winnow::prelude::*;

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

pub fn expression_parser<'a>(input: &mut &'a str) -> PResult<Expression<'a>> {
    alt((
        usages_parser.map(Expression::Usages),
        usage_tagged_parser.map(Expression::UsageTagged),
    ))
    .context(StrContext::Label("expression"))
    .parse_next(input)
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

#[derive(Debug, Error)]
pub enum KitaError {
    #[error("Usages list must be increasing")]
    IncreasingUsagesList,
}

pub fn usages_parser<'a>(input: &mut &'a str) -> PResult<Usages<'a>> {
    alt((
        (
            tag_parser,
            separated::<_, _, Vec<UsageItem>, _, _, _, _>(2.., usage_item_parser, ws_parser),
        )
            .try_map(|(tag, usage_items)| {
                // validate that integers are increasing with step 1
                let is_increasing = usage_items
                    .iter()
                    .enumerate()
                    .all(|(i, val)| val.1 == i as u8 + 1);
                if !is_increasing {
                    return Err(KitaError::IncreasingUsagesList);
                }
                Ok(Usages::Common(tag, usage_items))
            }),
        separated::<_, _, Vec<UsageItemTagged>, _, _, _, _>(
            2..,
            usage_item_tagged_parser,
            ws_parser,
        )
        .try_map(|usage_items_tagged| {
            // validate that integers are increasing with step 1
            let is_increasing = usage_items_tagged
                .iter()
                .enumerate()
                .all(|(i, val)| val.1 == i as u8 + 1);
            if !is_increasing {
                return Err(KitaError::IncreasingUsagesList);
            }
            Ok(Usages::Individual(usage_items_tagged))
        }),
    ))
    .context(StrContext::Label("usages"))
    .parse_next(input)
}

/*
UsageItemTagged(i)
    i "." ws UsageTagged
*/
#[derive(Debug)]
pub struct UsageItemTagged<'a>(UsageTagged<'a>, Index);

pub fn usage_item_tagged_parser<'a>(input: &mut &'a str) -> PResult<UsageItemTagged<'a>> {
    separated_pair(
        integer_parser,
        terminated('.', ws_parser),
        usage_tagged_parser,
    )
    .map(|(index, usage_tagged)| UsageItemTagged(usage_tagged, index))
    .context(StrContext::Label("usage_item_tagged"))
    .parse_next(input)
}

/*
UsageTagged
    Tag Usage
*/
#[derive(Debug)]
pub struct UsageTagged<'a>(Tag, Usage<'a>);

pub fn usage_tagged_parser<'a>(input: &mut &'a str) -> PResult<UsageTagged<'a>> {
    (tag_parser, usage_parser)
        .map(|(tag, usage)| UsageTagged(tag, usage))
        .context(StrContext::Label("usage_tagged"))
        .parse_next(input)
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

pub fn tag_parser<'a>(input: &mut &'a str) -> PResult<Tag> {
    (
        opt(terminated(part_of_speech_tag_parser, ws_parser)),
        opt(terminated(categories_parser, ws_parser)),
        opt(terminated(temporality_parser, ws_parser)),
    )
        .map(|(part_of_speech, categories, temporality)| {
            Tag(part_of_speech, categories, temporality)
        })
        .context(StrContext::Label("tag"))
        .parse_next(input)
}

/*
UsageItem(i)
    i "." ws Usage
*/
#[derive(Debug)]
pub struct UsageItem<'a>(Usage<'a>, Index);

pub fn usage_item_parser<'a>(input: &mut &'a str) -> PResult<UsageItem<'a>> {
    separated_pair(integer_parser, terminated('.', ws_parser), usage_parser)
        .map(|(index, usage)| UsageItem(usage, index))
        .context(StrContext::Label("usage_item"))
        .parse_next(input)
}

/*
Usage
    Definition (";" ws Definition)*
*/
// todo: make DefinitionItem indirection, how to get index without state?
//pub struct DefinitionItem<'a>(Definition<'a>, Index);
#[derive(Debug)]
pub struct Usage<'a>(Vec<Definition<'a>>);

pub fn usage_parser<'a>(input: &mut &'a str) -> PResult<Usage<'a>> {
    separated(1.., definition_parser, terminated(';', ws_parser))
        .map(Usage)
        .context(StrContext::Label("usage"))
        .parse_next(input)
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

pub fn definition_parser<'a>(input: &mut &'a str) -> PResult<Definition<'a>> {
    alt((
        reference_parser.map(Definition::Reference),
        sentence_de_parser.map(Definition::SentenceDe),
    ))
    .context(StrContext::Label("definition"))
    .parse_next(input)
}
