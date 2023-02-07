use nom::{
    combinator::{map, opt},
    error::context,
    sequence::tuple,
    IResult,
};
use nom_supreme::error::ErrorTree;

use super::super::general::character::superscript_number_parser;
use super::super::general::sentence_ka::root_ka_parser;
use super::{super::general::Index, character::infinitive_suffix_parser};

/*
VerbTermInfinitive
  RootKa InfinitiveSuffix SuperscriptNumber?
*/
#[derive(Debug)]
/// root, infinitive suffix, index
pub struct VerbTermInfinitive<'a>(&'a str, &'a str, Option<Index>);

pub fn term_infinitive_parser(input: &str) -> IResult<&str, VerbTermInfinitive, ErrorTree<&str>> {
    context(
        "term_infinitive",
        map(
            tuple((
                root_ka_parser,
                infinitive_suffix_parser,
                opt(superscript_number_parser),
            )),
            |(value, suffix, index)| VerbTermInfinitive(value, suffix, index),
        ),
    )(input)
}

/*
VerbTerm
    RootKa SuperscriptNumber?
*/
#[derive(Debug)]
pub struct VerbTerm<'a>(&'a str, Option<Index>);

pub fn term_parser(input: &str) -> IResult<&str, VerbTerm, ErrorTree<&str>> {
    context(
        "term",
        map(
            tuple((root_ka_parser, opt(superscript_number_parser))),
            |(value, index)| VerbTerm(value, index),
        ),
    )(input)
}
