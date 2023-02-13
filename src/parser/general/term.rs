use super::character::superscript_number_parser;
use super::sentence_ka::{word_ka_parser, WordKa};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::error::{context, VerboseError};
use nom::sequence::{tuple, terminated};
use nom::IResult;

use super::Index;

/*
Term
  HeadwordKa SuperscriptNumber?
*/
#[derive(Debug)]
pub struct Term<'a>(HeadwordKa<'a>, Option<Index>);

pub fn term_parser(input: &str) -> IResult<&str, Term, VerboseError<&str>> {
    context(
        "term",
        map(
            tuple((headword_ka_parser, opt(superscript_number_parser))),
            |(value, index)| Term(value, index),
        ),
    )(input)
}

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

pub fn headword_ka_parser(input: &str) -> IResult<&str, HeadwordKa, VerboseError<&str>> {
    context(
        "headword_ka",
        alt((
            map(
                terminated(word_ka_parser, char('!')),
                HeadwordKa::Exclamation,
            ),
            map(word_ka_parser, HeadwordKa::Normal),
        )),
    )(input)
}