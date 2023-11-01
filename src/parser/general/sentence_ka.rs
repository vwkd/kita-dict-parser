use winnow::combinator::{delimited, opt, separated_pair, terminated};
use winnow::prelude::*;
use winnow::{combinator::alt, error::StrContext};

use super::{word_ka::word_ka_small_parser, Value};

/*
HeadwordKa
  WordKa "!"
  WordKa
*/
#[derive(Debug)]
pub enum HeadwordKa<'a> {
    Normal(WordKa<'a>),
    Exclamation(WordKa<'a>),
}

pub fn headword_ka_parser<'a>(input: &mut &'a str) -> PResult<HeadwordKa<'a>> {
    alt((
        terminated(word_ka_parser, '!').map(HeadwordKa::Exclamation),
        word_ka_parser.map(HeadwordKa::Normal),
    ))
    .context(StrContext::Label("headword_ka"))
    .parse_next(input)
}

/*
WordKa
  WordKaRoot "-" WordKaRoot
  WordKaRoot
  WordKaPlain
*/
#[derive(Debug)]
pub enum WordKa<'a> {
    Plain(&'a str),
    Root(WordKaRoot<'a>),
    /// hyphenated
    TwoRoot(WordKaRoot<'a>, WordKaRoot<'a>),
}

pub fn word_ka_parser<'a>(input: &mut &'a str) -> PResult<WordKa<'a>> {
    alt((
        separated_pair(word_ka_root_parser, '-', word_ka_root_parser)
            .map(|(first, second)| WordKa::TwoRoot(first, second)),
        word_ka_root_parser.map(WordKa::Root),
        word_ka_plain_parser.map(WordKa::Plain),
    ))
    .context(StrContext::Label("word_ka"))
    .parse_next(input)
}

/*
WordKaPlain
  WordKaSmall "-" WordKaSmall
  WordKaSmall "-"
  WordKaSmall
  "-" WordKaSmall
*/
pub fn word_ka_plain_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        (word_ka_small_parser, '-', word_ka_small_parser).recognize(),
        (word_ka_small_parser, '-').recognize(),
        word_ka_small_parser,
        ('-', word_ka_small_parser).recognize(),
    ))
    .context(StrContext::Label("word_ka_plain"))
    .parse_next(input)
}

/*
WordKaRoot
  WordKaSmall? RootKa WordKaSmall?
*/
#[derive(Debug)]
/// srtart, root, end
pub struct WordKaRoot<'a>(Option<Value<'a>>, Value<'a>, Option<Value<'a>>);

pub fn word_ka_root_parser<'a>(input: &mut &'a str) -> PResult<WordKaRoot<'a>> {
    (
        opt(word_ka_small_parser),
        root_ka_parser,
        opt(word_ka_small_parser),
    )
        .map(|(start, root, end)| WordKaRoot(start, root, end))
        .context(StrContext::Label("wort_ka_root"))
        .parse_next(input)
}

/*
RootKa
  "**" WordKaSmall "**"
*/
pub fn root_ka_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    delimited("**", word_ka_small_parser, "**")
        .context(StrContext::Label("root_ka"))
        .parse_next(input)
}
