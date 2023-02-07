use super::{word_ka::word_ka_small_parser, Value};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, recognize},
    error::context,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};
use nom_supreme::error::ErrorTree;

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

pub fn headword_ka_parser(input: &str) -> IResult<&str, HeadwordKa, ErrorTree<&str>> {
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

pub fn word_ka_parser(input: &str) -> IResult<&str, WordKa, ErrorTree<&str>> {
    context(
        "word_ka",
        alt((
            map(
                separated_pair(word_ka_root_parser, char('-'), word_ka_root_parser),
                |(first, second)| WordKa::TwoRoot(first, second),
            ),
            map(word_ka_root_parser, WordKa::Root),
            map(word_ka_plain_parser, WordKa::Plain),
        )),
    )(input)
}

/*
WordKaPlain
  WordKaSmall "-" WordKaSmall
  WordKaSmall "-"
  WordKaSmall
  "-" WordKaSmall
*/
pub fn word_ka_plain_parser(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    context(
        "word_ka_plain",
        alt((
            recognize(tuple((
                word_ka_small_parser,
                char('-'),
                word_ka_small_parser,
            ))),
            recognize(tuple((word_ka_small_parser, char('-')))),
            word_ka_small_parser,
            recognize(tuple((char('-'), word_ka_small_parser))),
        )),
    )(input)
}

/*
WordKaRoot
  WordKaSmall RootKa WordKaSmall
  WordKaSmall RootKa
  RootKa WordKaSmall
*/
#[derive(Debug)]
/// srtart, root, end
pub struct WordKaRoot<'a>(Option<Value<'a>>, Value<'a>, Option<Value<'a>>);

pub fn word_ka_root_parser(input: &str) -> IResult<&str, WordKaRoot, ErrorTree<&str>> {
    context(
        "wort_ka_root",
        alt((
            map(
                tuple((word_ka_small_parser, root_ka_parser, word_ka_small_parser)),
                |(start, root, end)| WordKaRoot(Some(start), root, Some(end)),
            ),
            map(
                tuple((word_ka_small_parser, root_ka_parser)),
                |(start, root)| WordKaRoot(Some(start), root, None),
            ),
            map(
                tuple((root_ka_parser, word_ka_small_parser)),
                |(root, end)| WordKaRoot(None, root, Some(end)),
            ),
        )),
    )(input)
}

/*
RootKa
  "**" WordKaSmall "**"
*/
pub fn root_ka_parser(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    context(
        "root_ka",
        delimited(tag("**"), word_ka_small_parser, tag("**")),
    )(input)
}
