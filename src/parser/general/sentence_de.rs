use super::{
    character::{integer_parser, ws_parser},
    sentence_ka::word_ka_plain_parser,
    word_de::{word_de_big_parser, word_de_small_parser},
    word_ka::word_ka_small_parser,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{not, opt, recognize},
    error::{context, VerboseError},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

/*
SentenceDe
  SentenceDePart (SentenceDeSeparator SentenceDePart)*
*/
pub fn sentence_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "sentence_de",
        recognize(separated_list1(
            sentence_de_separator_parser,
            sentence_de_part_parser,
        )),
    )(input)
}

/*
SentenceDePart
  WordsDe
  '"' WordsDe '"'
  "(" Explanation ")"
  "(" WordsDe ")"
*/
pub fn sentence_de_part_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "sentence_de_part",
        alt((
            words_de_parser,
            recognize(delimited(char('"'), words_de_parser, char('"'))),
            recognize(delimited(char('('), explanation_parser, char(')'))),
            recognize(delimited(char('('), words_de_parser, char(')'))),
        )),
    )(input)
}

/*
SentenceDeSeparator
  ws
  "," ws
*/
pub fn sentence_de_separator_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "sentence_de_separator",
        alt((
            recognize(ws_parser),
            recognize(terminated(char(','), ws_parser)),
        )),
    )(input)
}

/*
Explanation
  ExplanationTag ws "für" (ws WordKaPlain)+ "!"?
*/
pub fn explanation_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "explanation",
        recognize(preceded(
            tuple((explanation_tag_parser, ws_parser, tag("für"), ws_parser)),
            terminated(
                separated_list1(ws_parser, word_ka_plain_parser),
                opt(char('!')),
            ),
        )),
    )(input)
}

/*
ExplanationTag
  "Abk."
  "umg."
*/
pub fn explanation_tag_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("explanation_tag", alt((tag("Abk."), tag("umg."))))(input)
}

/*
WordsDe
  WordDe (WordDeSeparator WordDe)*
*/
pub fn words_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "words_de",
        recognize(separated_list1(word_de_separator_parser, word_de_parser)),
    )(input)
}

/*
WordDeSeparator
  ws
  "," ws
  "/"
*/
pub fn word_de_separator_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "word_de_separator",
        alt((
            recognize(ws_parser),
            recognize(terminated(char(','), ws_parser)),
            recognize(char('/')),
        )),
    )(input)
}

/*
WordDe
  Integer
  ShorthandDe
  ShorthandOtherDe
  WordDeSmall "!"
  WordDeSmall "-" WordDeSmall
  WordDeSmall "-"
  WordDeSmall
  "-" WordDeSmall
  "(" WordDeSmall ")" WordDeSmall "-"
  WordDeBig "-" WordDeBig
  WordDeBig "-" WordDeSmall
  WordDeBig "-"
  WordDeBig "(" WordDeSmall ")"
  WordDeBig "..."
  WordDeBig
  "(" WordDeBig "-" ")" WordDeBig
  WordKaSmall "-" WordDeBig
  // ...
*/
pub fn word_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "word_de",
        alt((
            // beware: negative lookahead for ".", otherwise consumes part of higher-up UsageItem which then fails
            recognize(terminated(integer_parser, not(char('.')))),
            shorthand_de_parser,
            shorthand_other_de_parser,
            recognize(pair(word_de_small_parser, char('!'))),
            recognize(tuple((
                word_de_small_parser,
                char('-'),
                word_de_small_parser,
            ))),
            recognize(tuple((word_de_small_parser, char('-')))),
            word_de_small_parser,
            recognize(pair(char('-'), word_de_small_parser)),
            recognize(tuple((
                char('('),
                word_de_small_parser,
                char(')'),
                word_de_small_parser,
                char('-'),
            ))),
            recognize(tuple((word_de_big_parser, char('-'), word_de_big_parser))),
            recognize(tuple((word_de_big_parser, char('-'), word_de_small_parser))),
            recognize(pair(word_de_big_parser, char('-'))),
            recognize(tuple((
                word_de_big_parser,
                char('('),
                word_de_small_parser,
                char(')'),
            ))),
            recognize(pair(word_de_big_parser, tag("..."))),
            word_de_big_parser,
            recognize(tuple((
                char('('),
                word_de_big_parser,
                char('-'),
                char(')'),
                word_de_big_parser,
            ))),
            recognize(tuple((word_ka_small_parser, char('-'), word_de_big_parser))),
            // ...
        )),
    )(input)
}

/*
ShorthandOtherDe
  "ca."
  "durch-ea."
  "kaukas."
  "NG"
  "od."
  "OG"
  "SG"
  "WG"
  "z.B."
*/
pub fn shorthand_other_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "shorthand_other_de",
        alt((
            tag("ca."),
            tag("durch-ea."),
            tag("kaukas."),
            tag("NG"),
            tag("od."),
            tag("OG"),
            tag("SG"),
            tag("WG"),
            tag("z.B."),
        )),
    )(input)
}

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
pub fn shorthand_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "shorthand_de",
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
                recognize(separated_pair(tag("u."), ws_parser, tag("zw."))),
                tag("v."),
                tag("wg."),
                tag("zs."),
            )),
        )),
    )(input)
}
