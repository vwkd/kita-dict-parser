use super::{
    category::category_parser,
    character::{integer_parser, ws_parser},
    part_of_speech::part_of_speech_parser,
    word_de::{word_de_big_parser, word_de_small_parser},
    word_ka::word_ka_small_parser,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{not, recognize},
    error::{context, VerboseError},
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair, terminated, tuple},
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
  "(" WordsDe ")"
*/
pub fn sentence_de_part_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "sentence_de_part",
        alt((
            words_de_parser,
            recognize(delimited(char('"'), words_de_parser, char('"'))),
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
  DateDe
  Integer
  ShorthandCombinationDe
  ShorthandDe
  ShorthandOtherDe
  WordDeSmall "(" WordDeSmall ")"
  WordDeSmall "!"
  WordDeSmall "-" WordDeSmall
  WordDeSmall "-"
  WordDeSmall ":"
  WordDeSmall
  "-" WordDeSmall
  "(" WordDeSmall ")" WordDeSmall "-"
  "(" WordDeSmall ")" WordDeSmall
  WordDeBig "-" WordDeBig
  WordDeBig "-" WordDeSmall
  WordDeBig "-"
  WordDeBig "(" WordDeSmall ")"
  WordDeBig "..."
  WordDeBig
  "(" WordDeBig "-" ")" WordDeBig
  WordKaSmall "-" WordDeBig
  WordKaSmall "!"
  WordKaSmall
  // ...
*/
pub fn word_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "word_de",
        alt((
            alt((
                recognize(date_de_parser),
                // beware: negative lookahead for ".", otherwise consumes part of higher-up UsageItem which then fails
                recognize(terminated(integer_parser, not(char('.')))),
                shorthand_combination_de_parser,
                shorthand_de_parser,
                shorthand_other_de_parser,
                recognize(tuple((
                    word_de_small_parser,
                    char('('),
                    word_de_small_parser,
                    char(')'),
                ))),
                recognize(pair(word_de_small_parser, char('!'))),
                recognize(tuple((
                    word_de_small_parser,
                    char('-'),
                    word_de_small_parser,
                ))),
                recognize(tuple((word_de_small_parser, char('-')))),
                recognize(tuple((word_de_small_parser, char(':')))),
                word_de_small_parser,
                recognize(pair(char('-'), word_de_small_parser)),
                recognize(tuple((
                    char('('),
                    word_de_small_parser,
                    char(')'),
                    word_de_small_parser,
                    char('-'),
                ))),
                recognize(tuple((
                    char('('),
                    word_de_small_parser,
                    char(')'),
                    word_de_small_parser,
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
            )),
            alt((
                recognize(tuple((
                    char('('),
                    word_de_big_parser,
                    char('-'),
                    char(')'),
                    word_de_big_parser,
                ))),
                recognize(tuple((word_ka_small_parser, char('-'), word_de_big_parser))),
                recognize(pair(word_ka_small_parser, char('!'))),
                word_ka_small_parser,
            )),
            // ...
        )),
    )(input)
}

/*
ShorthandCombinationDe
  "a." ws Category
  "a." ws PartOfSpeech
  "imS" ":"
*/
pub fn shorthand_combination_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "shorthand_combination_de",
        alt((
            recognize(separated_pair(tag("a."), ws_parser, category_parser)),
            recognize(separated_pair(tag("a."), ws_parser, part_of_speech_parser)),
            recognize(tuple((tag("imS"), char(':')))),
        )),
    )(input)
}

/*
ShorthandOtherDe
  "ca."
  "d.h."
  "durch-ea."
  "kaukas."
  "NG"
  "od."
  "OG"
  "SG"
  "umg."
  "WG"
  "z.B."
*/
pub fn shorthand_other_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "shorthand_other_de",
        alt((
            tag("ca."),
            tag("d.h."),
            tag("durch-ea."),
            tag("kaukas."),
            tag("NG"),
            tag("od."),
            tag("OG"),
            tag("SG"),
            tag("umg."),
            tag("WG"),
            tag("z.B."),
        )),
    )(input)
}

/*
ShorthandDe
  "a."
  "Abk."
  "ag."
  "b."
  "Bed."
  "d.O."
  "do."
  "DOZ"
  "e-e"
  "e-m"
  "e-n"
  "e-r"
  "e-s"
  "e."
  "ea."
  "ehm."
  "et."
  "Fn."
  "FR"
  "fr."
  "g."
  "gebr."
  "Ggs."
  "i.O."
  "imS"
  "intr."
  "IOZ"
  "Iter."
  "j-d"
  "j-m"
  "j-n"
  "j-s"
  "L."
  "l."
  "m-e"
  "m-m"
  "m-n"
  "m-r"
  "m-s"
  "m."
  "m. Vn."
  "mst"
  "neg."
  "NG"
  "ng."
  "NV"
  "Obj."
  "od."
  "OG"
  "og."
  "OR"
  "OV"
  "OVZ"
  "P."
  "Pkt."
  "PR"
  "s."
  "SG"
  "sg."
  "sn"
  "Subj."
  "SupV"
  "SupVZ"
  "SV"
  "T."
  "u. zw."
  "u."
  "v."
  "Vn."
  "w. Vn."
  "WG"
  "wg."
  "zs."
*/
pub fn shorthand_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "shorthand_de",
        alt((
            alt((
                tag("a."),
                tag("Abk."),
                tag("ag."),
                tag("b."),
                tag("Bed."),
                tag("d.O."),
                tag("do."),
                tag("DOZ"),
                tag("e-e"),
                tag("e-m"),
                tag("e-n"),
                tag("e-r"),
                tag("e-s"),
                tag("e."),
                tag("ea."),
                tag("ehm."),
                tag("et."),
                tag("Fn."),
                tag("FR"),
                tag("fr."),
            )),
            alt((
                tag("g."),
                tag("gebr."),
                tag("Ggs."),
                tag("i.O."),
                tag("imS"),
                tag("intr."),
                tag("IOZ"),
                tag("Iter."),
                tag("j-d"),
                tag("j-m"),
                tag("j-n"),
                tag("j-s"),
                tag("L."),
                tag("l."),
                tag("m-e"),
                tag("m-m"),
                tag("m-n"),
                tag("m-r"),
                tag("m-s"),
                tag("m."),
            )),
            alt((
                tag("m. Vn."),
                tag("mst"),
                tag("neg."),
                tag("NG"),
                tag("ng."),
                tag("NV"),
                tag("Obj."),
                tag("od."),
                tag("OG"),
                tag("og."),
                tag("OR"),
                tag("OV"),
                tag("OVZ"),
                tag("P."),
                tag("Pkt."),
                tag("PR"),
                tag("s."),
                tag("SG"),
                tag("sg."),
                tag("sn"),
            )),
            alt((
                tag("Subj."),
                tag("SupV"),
                tag("SupVZ"),
                tag("SV"),
                tag("T."),
                tag("u. zw."),
                tag("u."),
                tag("v."),
                tag("Vn."),
                tag("w. Vn."),
                tag("WG"),
                tag("wg."),
                tag("zs."),
            )),
        )),
    )(input)
}

// note: don't validate day
/*
DateDe
  Integer "." "ws" MonthDe
*/
pub fn date_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "date_de",
        recognize(separated_pair(
            terminated(integer_parser, char('.')),
            ws_parser,
            month_de_parser,
        )),
    )(input)
}

/*
MonthDe
  "Januar"
  "Februar"
  "März"
  "April"
  "Mai"
  "Juni"
  "Juli"
  "August"
  "Sept."
  "Okt."
  "Nov."
  "Dez."
*/
pub fn month_de_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "month_de",
        alt((
            tag("Januar"),
            tag("Februar"),
            tag("März"),
            tag("April"),
            tag("Mai"),
            tag("Juni"),
            tag("Juli"),
            tag("August"),
            tag("Sept."),
            tag("Okt."),
            tag("Nov."),
            tag("Dez."),
        )),
    )(input)
}
