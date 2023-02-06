use super::{
    character::ws_parser,
    word_de::{word_de_big_parser, word_de_small_parser},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::recognize,
    error::ParseError,
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

/*
SentenceDe
  SentenceDePart (ws SentenceDePart)*
*/
pub fn sentence_de_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    recognize(separated_list1(
        ws_parser,
        sentence_de_part_parser,
    ))(input)
}

/*
SentenceDePart
  "(" WordsDe ")"
  WordsDe
*/
pub fn sentence_de_part_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    alt((
        recognize(delimited(char('('), words_de_parser, char(')'))),
        words_de_parser,
    ))(input)
}

/*
WordsDe
  WordDe (SeparatorDe WordDe)*
*/
pub fn words_de_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    recognize(separated_list1(separator_de_parser, word_de_parser))(input)
}

/*
SeparatorDe
  ws
  "," ws
  "/"
*/
pub fn separator_de_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    alt((
        recognize(ws_parser),
        recognize(tag(", ")),
        recognize(char('/')),
    ))(input)
}

/*
WordDe
  Digit+
  ShorthandDe
  WordDeSmall "!"
  WordDeSmall "-"
  WordDeSmall
  "-" WordDeSmall
  "(" WordDeSmall ")" WordDeSmall "-"
  WordDeBig "-" WordDeSmall
  WordDeBig "(" WordDeSmall ")"
  WordDeBig
  // ...
*/
pub fn word_de_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    alt((
        digit1,
        shorthand_de_parser,
        recognize(pair(word_de_small_parser, char('!'))),
        recognize(tuple((word_de_small_parser, char('-')))),
        word_de_small_parser,
        recognize(tuple((char('-'), word_de_small_parser))),
        recognize(tuple((
            char('('),
            word_de_small_parser,
            char(')'),
            word_de_small_parser,
            char('-'),
        ))),
        recognize(tuple((word_de_big_parser, char('-'), word_de_small_parser))),
        recognize(tuple((
            word_de_big_parser,
            char('('),
            word_de_small_parser,
            char(')'),
        ))),
        word_de_big_parser,
        // ...
    ))(input)
}

/*
ShorthandDe
  "z.B."
*/
pub fn shorthand_de_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    alt((
        tag("z.B."),
        //
    ))(input)
}
