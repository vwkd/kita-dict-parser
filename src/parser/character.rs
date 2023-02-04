use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::error::ParseError;
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
