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

use winnow::combinator::separated_pair;
use winnow::error::StrContext;
use winnow::prelude::*;

use character::ws_parser;
use expression::{expression_parser, Expression};

use term::{term_parser, Term};

pub type Value<'a> = &'a str;
pub type Index = u8;

/*
Parser
  Entry EOF
*/
pub fn parse(input: &mut str) -> Result<Entry, String> {
    entry_parser.parse(input).map_err(|e| e.to_string())
}

/*
Entry
  Term ws Expression
*/
#[derive(Debug)]
pub struct Entry<'a>(Term<'a>, Expression<'a>);

pub fn entry_parser<'a>(input: &mut &'a str) -> PResult<Entry<'a>> {
    separated_pair(term_parser, ws_parser, expression_parser)
        .map(|(t, e)| Entry(t, e))
        .context(StrContext::Label("entry"))
        .parse_next(input)
}
