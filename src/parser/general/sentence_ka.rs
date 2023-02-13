use super::{word_ka::word_ka_small_parser, Value};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt, recognize},
    error::{context, VerboseError},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

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

pub fn word_ka_parser(input: &str) -> IResult<&str, WordKa, VerboseError<&str>> {
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
pub fn word_ka_plain_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
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
  WordKaSmall? RootKa WordKaSmall?
*/
#[derive(Debug)]
/// srtart, root, end
pub struct WordKaRoot<'a>(Option<Value<'a>>, Value<'a>, Option<Value<'a>>);

pub fn word_ka_root_parser(input: &str) -> IResult<&str, WordKaRoot, VerboseError<&str>> {
    context(
        "wort_ka_root",
        map(
            tuple((
                opt(word_ka_small_parser),
                root_ka_parser,
                opt(word_ka_small_parser),
            )),
            |(start, root, end)| WordKaRoot(start, root, end),
        ),
    )(input)
}

/*
RootKa
  "**" WordKaSmall "**"
*/
pub fn root_ka_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "root_ka",
        delimited(tag("**"), word_ka_small_parser, tag("**")),
    )(input)
}
