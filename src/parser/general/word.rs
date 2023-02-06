use std::num::ParseIntError;

use super::{
    character::{integer_parser, ws_parser},
    word_de::{word_de_big_parser, word_de_small_parser},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::recognize,
    error::{FromExternalError, ParseError},
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

/*
SentenceDe
  SentenceDePart (ws SentenceDePart)*
*/
pub fn sentence_de_parser<'i, E>(input: &'i str) -> IResult<&'i str, &'i str, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
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
pub fn sentence_de_part_parser<'i, E>(input: &'i str) -> IResult<&'i str, &'i str, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    alt((
        recognize(delimited(char('('), words_de_parser, char(')'))),
        words_de_parser,
    ))(input)
}

/*
WordsDe
  WordDe (SeparatorDe WordDe)*
*/
pub fn words_de_parser<'i, E>(input: &'i str) -> IResult<&'i str, &'i str, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
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
  Integer
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
pub fn word_de_parser<'i, E>(input: &'i str) -> IResult<&'i str, &'i str, E>
where
    E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
{
    alt((
        recognize(integer_parser),
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

// todo: add missing, e.g. `z.B.`
/*
ShorthandDe
  "b."
  "e."
  "ea."
  "e-e"
  "e-m"
  "e-n"
  "e-r"
  "e-s"
  "et."
  "fr."
  "g."
  "j-d"
  "j-m"
  "j-n"
  "j-s"
  "l."
  "m."
  "m-e"
  "m-m"
  "m-n"
  "m-r"
  "m-s"
  "ng."
  "og."
  "u."
  "u. zw."
  "v."
  "wg."
  "zs."

  // "a."
  // "Abk."
  // "ag."
  // "Bed."
  // "do."
  // "d.O."
  // "DOZ"
  // "ehm."
  // "Fn."
  // "FR"
  // "gebr."
  // "Ggs."
  // "imS"
  // "intr."
  // "i.O."
  // "IOZ"
  // "Iter."
  // "L."
  // "mst"
  // "m. Vn."
  // "neg."
  // "NG"
  // "NV"
  // "Obj."
  // "od."
  // "OG"
  // "OR"
  // "OV"
  // "OVZ"
  // "P."
  // "Pkt."
  // "PR"
  // "s."
  // "sg."
  // "SG"
  // "sn"
  // "Subj."
  // "SupV"
  // "SupVZ"
  // "SV"
  // "T."
  // "Vn."
  // "WG"
  // "w. Vn."
*/
pub fn shorthand_de_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    alt((
        alt((
            tag("b."),
            tag("e."),
            tag("ea."),
            tag("e-e"),
            tag("e-m"),
            tag("e-n"),
            tag("e-r"),
            tag("e-s"),
            tag("et."),
            tag("fr."),
            tag("g."),
            tag("j-d"),
            tag("j-m"),
            tag("j-n"),
            tag("j-s"),
            tag("l."),
            tag("m."),
            tag("m-e"),
            tag("m-m"),
        )),
        alt((
            tag("m-n"),
            tag("m-r"),
            tag("m-s"),
            tag("ng."),
            tag("og."),
            tag("u."),
            tag("u. zw."),
            tag("v."),
            tag("wg."),
            tag("zs."),
        )),
    ))(input)
}
