use super::{character::ws_parser, word_de::word_de_parser, word_ka::word_ka_parser};
use nom::{branch::alt, combinator::recognize, error::ParseError, multi::separated_list1, IResult};

/*
Words
    Word (ws Word)*
*/
pub fn words_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    recognize(separated_list1(ws_parser, word_parser))(input)
}

/*
Word
    WordDe
    WordKa
*/
pub fn word_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    alt((word_de_parser, word_ka_parser))(input)
}
