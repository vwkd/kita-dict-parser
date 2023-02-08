use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::map,
    error::{context, VerboseError},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

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

pub fn conjugation_parser(input: &str) -> IResult<&str, VerbConjugation, VerboseError<&str>> {
    context(
        "conjugation",
        map(
            separated_pair(form_class1_parser, ws_parser, form_class23_parser),
            |((present_s1, future_s1), (aorist_s1, perfective_s1))| {
                VerbConjugation(present_s1, future_s1, aorist_s1, perfective_s1)
            },
        ),
    )(input)
}

/*
VerbFormClass1
  WordKaSmall ws "fut" ws WordKaSmall
  WordKaSmall "," ws WordKaSmall
*/
pub fn form_class1_parser(input: &str) -> IResult<&str, (PresentS1, FutureS1), VerboseError<&str>> {
    context(
        "form_class1",
        alt((
            separated_pair(
                map(word_ka_small_parser, PresentS1::Full),
                delimited(ws_parser, tag("fut"), ws_parser),
                map(word_ka_small_parser, FutureS1::Full),
            ),
            separated_pair(
                map(word_ka_small_parser, PresentS1::Full),
                terminated(char(','), ws_parser),
                map(terminated(preverb_parser, char('~')), FutureS1::Preverb),
            ),
        )),
    )(input)
}

/*
VerbFormClass23
  "(" WordKaSmall ", " WordKaSmall ")"
*/
pub fn form_class23_parser(
    input: &str,
) -> IResult<&str, (AoristS1, PerfectiveS1), VerboseError<&str>> {
    context(
        "form_class23",
        delimited(
            char('('),
            separated_pair(
                map(word_ka_small_parser, AoristS1::Full),
                terminated(char(','), ws_parser),
                map(word_ka_small_parser, PerfectiveS1::Full),
            ),
            char(')'),
        ),
    )(input)
}
