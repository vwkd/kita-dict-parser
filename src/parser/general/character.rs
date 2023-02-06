use std::num::ParseIntError;

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map_res, recognize, value};
use nom::error::{FromExternalError, ParseError};
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

/*
ws
    UNICODE_WHITESPACE_CHARACTER
*/
pub fn ws_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, char, E> {
    char(' ')(input)
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
pub fn superscript_number_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, u8, E> {
    alt((
        value(1, char('¹')),
        value(2, char('²')),
        value(3, char('³')),
        value(4, char('⁴')),
        value(5, char('⁵')),
        value(6, char('⁶')),
        value(7, char('⁷')),
        value(8, char('⁸')),
        value(9, char('⁹')),
    ))(input)
}

/*
Integer
    DigitNonZero (Digit)*
*/
pub fn integer_parser<'i, E>(input: &'i str) -> IResult<&'i str, u8, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    map_res(
        recognize(pair(digit_non_zero_parser, many0(digit_parser))),
        |s: &str| s.parse::<u8>(),
    )(input)
}

/*
Digit
    "0"
    DigitNonZero
*/
fn digit_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    alt((recognize(char('0')), recognize(digit_non_zero_parser)))(input)
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
fn digit_non_zero_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    recognize(alt((
        char('1'),
        char('2'),
        char('3'),
        char('4'),
        char('5'),
        char('6'),
        char('7'),
        char('8'),
        char('9'),
    )))(input)
}
