use winnow::combinator::{alt, delimited, terminated};
use winnow::prelude::*;
use winnow::{combinator::separated_pair, error::StrContext};

use crate::parser::general::{character::ws_parser, word_ka::word_ka_small_parser};

use super::character::preverb_parser;

/*
VerbConjugation
  VerbFormClass1 ws VerbFormClass23
*/
#[derive(Debug)]
pub struct VerbConjugation<'a>(PresentS1<'a>, FutureS1<'a>, AoristS1<'a>, PerfectiveS1<'a>);

#[derive(Debug)]
pub enum PresentS1<'a> {
    Full(&'a str),
    // ...
}

#[derive(Debug)]
pub enum FutureS1<'a> {
    Full(&'a str),
    Preverb(&'a str),
}

#[derive(Debug)]
pub enum AoristS1<'a> {
    Full(&'a str),
    // ...
}

#[derive(Debug)]
pub enum PerfectiveS1<'a> {
    Full(&'a str),
    // ...
}

pub fn conjugation_parser<'a>(input: &mut &'a str) -> PResult<VerbConjugation<'a>> {
    separated_pair(form_class1_parser, ws_parser, form_class23_parser)
        .map(|((present_s1, future_s1), (aorist_s1, perfective_s1))| {
            VerbConjugation(present_s1, future_s1, aorist_s1, perfective_s1)
        })
        .context(StrContext::Label("conjugation"))
        .parse_next(input)
}

/*
VerbFormClass1
  WordKaSmall ws "fut" ws WordKaSmall
  WordKaSmall "," ws WordKaSmall
*/
pub fn form_class1_parser<'a>(input: &mut &'a str) -> PResult<(PresentS1<'a>, FutureS1<'a>)> {
    alt((
        separated_pair(
            word_ka_small_parser.map(PresentS1::Full),
            delimited(ws_parser, "fut", ws_parser),
            word_ka_small_parser.map(FutureS1::Full),
        ),
        separated_pair(
            word_ka_small_parser.map(PresentS1::Full),
            terminated(',', ws_parser),
            terminated(preverb_parser, '~').map(FutureS1::Preverb),
        ),
    ))
    .context(StrContext::Label("form_class1"))
    .parse_next(input)
}

/*
VerbFormClass23
  "(" WordKaSmall ", " WordKaSmall ")"
*/
pub fn form_class23_parser<'a>(input: &mut &'a str) -> PResult<(AoristS1<'a>, PerfectiveS1<'a>)> {
    delimited(
        '(',
        separated_pair(
            word_ka_small_parser.map(AoristS1::Full),
            terminated(',', ws_parser),
            word_ka_small_parser.map(PerfectiveS1::Full),
        ),
        ')',
    )
    .context(StrContext::Label("form_class23"))
    .parse_next(input)
}
