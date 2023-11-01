use winnow::combinator::opt;
use winnow::error::StrContext;
use winnow::prelude::*;

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

pub fn term_infinitive_parser<'a>(input: &mut &'a str) -> PResult<VerbTermInfinitive<'a>> {
    (
        root_ka_parser,
        infinitive_suffix_parser,
        opt(superscript_number_parser),
    )
        .map(|(value, suffix, index)| VerbTermInfinitive(value, suffix, index))
        .context(StrContext::Label("term_infinitive"))
        .parse_next(input)
}

/*
VerbTerm
    RootKa SuperscriptNumber?
*/
#[derive(Debug)]
pub struct VerbTerm<'a>(&'a str, Option<Index>);

pub fn term_parser<'a>(input: &mut &'a str) -> PResult<VerbTerm<'a>> {
    (root_ka_parser, opt(superscript_number_parser))
        .map(|(value, index)| VerbTerm(value, index))
        .context(StrContext::Label("term"))
        .parse_next(input)
}
