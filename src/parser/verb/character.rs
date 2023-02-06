use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::recognize,
    error::ParseError,
    sequence::{pair, terminated},
    IResult,
};

use crate::parser::general::character::ws_parser;

/*
nlwsws
  "\n  "
*/
pub fn nlwsws_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    recognize(terminated(char('\n'), pair(ws_parser, ws_parser)))(input)
}

/*
Preverb
  "გა"
  // ...
*/
pub fn preverb_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    alt((
        tag("გა"),
        tag("გადა"),
        //
    ))(input)
}

/*
InfinitiveSuffix
  "ობა"
  // ...
*/
pub fn infinitive_suffix_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    alt((
        tag("ობა"),
        tag("ება"),
        //
    ))(input)
}
