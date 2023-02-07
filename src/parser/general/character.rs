use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map_res, recognize, value};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;
use nom_supreme::error::ErrorTree;

/*
ws
    UNICODE_WHITESPACE_CHARACTER
*/
pub fn ws_parser(input: &str) -> IResult<&str, char, ErrorTree<&str>> {
    context("ws", char(' '))(input)
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
pub fn superscript_number_parser(input: &str) -> IResult<&str, u8, ErrorTree<&str>> {
    context(
        "superscript_number",
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
        )),
    )(input)
}

/*
Integer
    DigitNonZero (Digit)*
*/
pub fn integer_parser(input: &str) -> IResult<&str, u8, ErrorTree<&str>> {
    context(
        "integer",
        map_res(
            recognize(pair(digit_non_zero_parser, many0(digit_parser))),
            |s: &str| s.parse::<u8>(),
        ),
    )(input)
}

/*
Digit
    "0"
    DigitNonZero
*/
fn digit_parser(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    context(
        "digit",
        alt((recognize(char('0')), recognize(digit_non_zero_parser))),
    )(input)
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
fn digit_non_zero_parser(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    context(
        "digit_non_zero",
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
        ))),
    )(input)
}
