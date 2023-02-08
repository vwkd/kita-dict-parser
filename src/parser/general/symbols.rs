use nom::{character::complete::char, combinator::value, error::VerboseError, IResult};

/*
Temporality
  "†"
*/
#[derive(Debug, Clone)]
pub enum Temporality {
    Archaic,
}

pub fn temporality_parser(input: &str) -> IResult<&str, Temporality, VerboseError<&str>> {
    value(Temporality::Archaic, char('†'))(input)
}
