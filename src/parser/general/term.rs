use super::character::superscript_number_parser;
use super::sentence_ka::{headword_ka_parser, HeadwordKa};
use nom::combinator::{map, opt};
use nom::error::{context, VerboseError};
use nom::sequence::tuple;
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
