use winnow::combinator::{alt, terminated};
use winnow::error::StrContext;
use winnow::prelude::*;

use crate::parser::general::character::ws_parser;

/*
nlwsws
  "\n  "
*/
pub fn nlwsws_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    terminated('\n', (ws_parser, ws_parser))
        .recognize()
        .context(StrContext::Label("nlwsws"))
        .parse_next(input)
}

/*
Preverb
  "გა"
  // ...
*/
pub fn preverb_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        "გა",
        "გადა",
        //
    ))
    .context(StrContext::Label("preverb"))
    .parse_next(input)
}

/*
InfinitiveSuffix
  "ობა"
  // ...
*/
pub fn infinitive_suffix_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        "ობა",
        "ება",
        //
    ))
    .context(StrContext::Label("infinitive_suffix"))
    .parse_next(input)
}
