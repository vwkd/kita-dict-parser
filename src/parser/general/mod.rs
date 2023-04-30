pub mod category;
pub mod character;
pub mod expression;
pub mod part_of_speech;
pub mod reference;
pub mod sentence_de;
pub mod sentence_ka;
pub mod symbols;
pub mod term;
pub mod word_de;
pub mod word_ka;

use character::ws_parser;
use expression::{expression_parser, Expression};
use nom::{
    combinator::map,
    error::{context, VerboseError},
    sequence::separated_pair,
    IResult, branch::alt,
};
use nom_supreme::final_parser::final_parser;
use term::{term_parser, Term};

pub type Value<'a> = &'a str;
pub type Index = u8;

/*
Parser
  Entry EOF
*/
pub fn parse(input: &str) -> Result<Entry, VerboseError<&str>> {
    final_parser(entry_parser)(input)
}

/*
Entry
  MultiEntry
  SingleEntry
*/
#[derive(Debug)]
pub enum Entry<'a> {
    Single(SingleEntry<'a>),
    Multiple(Vec<SingleEntry<'a>>),
}

pub fn entry_parser(input: &str) -> IResult<&str, Entry, VerboseError<&str>> {
    context(
        "entry",
        alt((
            map(multi_entry_parser, Entry::Multiple),
            map(single_entry_parser, Entry::Single),
        )),
    )(input)
}

/*
MultiEntry
  geoword(|geoword)? ws Expression (; ws geowords_including_tilde Expression)+
*/
pub fn multi_entry_parser(input: &str) -> IResult<&str, Vec<SingleEntry>, VerboseError<&str>> {

}

/*
SingleEntry
  Term ws Expression
*/
#[derive(Debug)]
pub struct SingleEntry<'a>(Term<'a>, Expression<'a>);

pub fn single_entry_parser(input: &str) -> IResult<&str, SingleEntry, VerboseError<&str>> {
    context(
        "single_entry",
        map(
            separated_pair(term_parser, ws_parser, expression_parser),
            |(t, e)| SingleEntry(t, e),
        ),
    )(input)
}
