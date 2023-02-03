use nom::character::complete::char;
use nom::error::ParseError;
use nom::{IResult, Parser};

/*
ws
  " "
*/
pub fn ws(input: &str) -> IResult<&str, char> {
    char(' ')(input)
}
