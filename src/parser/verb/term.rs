use nom::{
    combinator::{map, opt},
    error::ParseError,
    sequence::tuple,
    IResult,
};

use super::super::general::Index;
use super::super::general::{
    character::superscript_number_parser, word_ka::headword_ka_parser, word_ka::WordRootKa,
};

/*
VerbTerm
  HeadwordKa SuperscriptNumber?
*/
#[derive(Debug)]
pub struct VerbTerm<'a>(WordRootKa<'a>, Option<Index>);

pub fn term_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, VerbTerm, E> {
    map(
        tuple((headword_ka_parser, opt(superscript_number_parser))),
        |(value, index)| VerbTerm(value, index),
    )(input)
}
