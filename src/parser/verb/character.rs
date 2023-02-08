use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::recognize,
    error::{context, VerboseError},
    sequence::{pair, terminated},
    IResult,
};

use crate::parser::general::character::ws_parser;

/*
nlwsws
  "\n  "
*/
pub fn nlwsws_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "nlwsws",
        recognize(terminated(char('\n'), pair(ws_parser, ws_parser))),
    )(input)
}

/*
Preverb
  "გა"
  // ...
*/
pub fn preverb_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "preverb",
        alt((
            tag("გა"),
            tag("გადა"),
            //
        )),
    )(input)
}

/*
InfinitiveSuffix
  "ობა"
  // ...
*/
pub fn infinitive_suffix_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "infinitive_suffix",
        alt((
            tag("ობა"),
            tag("ება"),
            //
        )),
    )(input)
}
