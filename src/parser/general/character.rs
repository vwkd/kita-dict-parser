use winnow::prelude::*;
use winnow::{combinator::alt, combinator::repeat, error::StrContext, PResult};

/*
ws
    UNICODE_WHITESPACE_CHARACTER
*/
// pub fn entry_parser<'a>(input: &mut &'a str) -> PResult<Entry<'a>> {

pub fn ws_parser<'a>(input: &mut &'a str) -> PResult<char> {
    ' '.context(StrContext::Label("ws")).parse_next(input)
}

// todo: maybe more?
/*
SuperscriptNumber
    "¹"
    "²"
    "³"
    "⁴"
    "⁵"
    "⁶"
    "⁷"
    "⁸"
    "⁹"
*/
pub fn superscript_number_parser<'a>(input: &mut &'a str) -> PResult<u8> {
    alt((
        '¹'.value(1),
        '²'.value(2),
        '³'.value(3),
        '⁴'.value(4),
        '⁵'.value(5),
        '⁶'.value(6),
        '⁷'.value(7),
        '⁸'.value(8),
        '⁹'.value(9),
    ))
    .context(StrContext::Label("superscript_number"))
    .parse_next(input)
}

/*
Integer
    DigitNonZero (Digit)*
*/
pub fn integer_parser<'a>(input: &mut &'a str) -> PResult<u8> {
    (
        digit_non_zero_parser,
        repeat::<_, _, (), _, _>(0.., digit_parser),
    )
        .recognize()
        .context(StrContext::Label("integer"))
        .parse_to()
        .parse_next(input)
}

/*
Digit
    "0"
    DigitNonZero
*/
fn digit_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt(('0'.recognize(), digit_non_zero_parser.recognize()))
        .context(StrContext::Label("digit"))
        .parse_next(input)
}

/*
DigitNonZero
    "1"
    "2"
    "3"
    "4"
    "5"
    "6"
    "7"
    "8"
    "9"
*/
fn digit_non_zero_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt(('1', '2', '3', '4', '5', '6', '7', '8', '9'))
        .recognize()
        .context(StrContext::Label("digit_non_zero"))
        .parse_next(input)
}
