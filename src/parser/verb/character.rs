use nom::{branch::alt, bytes::complete::tag, error::ParseError, IResult};

/*
nltb
  "\n  "
*/
pub fn nltb_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    tag("\n  ")(input)
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
