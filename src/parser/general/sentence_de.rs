use winnow::combinator::{alt, delimited, not, separated, separated_pair};
use winnow::prelude::*;
use winnow::{combinator::terminated, error::StrContext};

use super::{
    category::category_parser,
    character::{integer_parser, ws_parser},
    part_of_speech::part_of_speech_parser,
    word_de::{word_de_big_parser, word_de_small_parser},
    word_ka::word_ka_small_parser,
};

/*
SentenceDe
  SentenceDePart (SentenceDeSeparator SentenceDePart)*
*/
pub fn sentence_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    separated::<_, _, (), _, _, _, _>(1.., sentence_de_part_parser, sentence_de_separator_parser)
        .recognize()
        .context(StrContext::Label("sentence_de"))
        .parse_next(input)
}

/*
SentenceDePart
  WordsDe
  '"' WordsDe '"'
  "(" WordsDeParentheses ")"
*/
pub fn sentence_de_part_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        words_de_parser,
        delimited('"', words_de_parser, '"').recognize(),
        delimited('(', words_de_parentheses_parser, ')').recognize(),
    ))
    .context(StrContext::Label("sentence_de_part"))
    .parse_next(input)
}

/*
SentenceDeSeparator
  ws
  "," ws
*/
pub fn sentence_de_separator_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        ws_parser.recognize(),
        terminated(',', ws_parser).recognize(),
    ))
    .context(StrContext::Label("sentence_de_separator"))
    .parse_next(input)
}

/*
WordsDeParentheses
  WordDe (WordDeParenthesesSeparator WordDe)*
*/
pub fn words_de_parentheses_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    separated::<_, _, (), _, _, _, _>(1.., word_de_parser, word_de_parentheses_separator_parser)
        .recognize()
        .context(StrContext::Label("words_de_parentheses"))
        .parse_next(input)
}

/*
WordDeParenthesesSeparator
  ws
  "," ws
  ";" ws
  "/"
*/
pub fn word_de_parentheses_separator_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        ws_parser.recognize(),
        terminated(',', ws_parser).recognize(),
        terminated(';', ws_parser).recognize(),
        '/'.recognize(),
    ))
    .context(StrContext::Label("word_de_parentheses_separator"))
    .parse_next(input)
}

/*
WordsDe
  WordDe (WordDeSeparator WordDe)*
*/
pub fn words_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    separated::<_, _, (), _, _, _, _>(1.., word_de_parser, word_de_separator_parser)
        .recognize()
        .context(StrContext::Label("words_de"))
        .parse_next(input)
}

/*
WordDeSeparator
  ws
  "," ws
  "/"
*/
pub fn word_de_separator_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        ws_parser.recognize(),
        terminated(',', ws_parser).recognize(),
        '/'.recognize(),
    ))
    .context(StrContext::Label("word_de_separator"))
    .parse_next(input)
}

/*
WordDe
  DateDe
  Integer
  ShorthandVariableDe
  ShorthandCombinationDe
  ShorthandDe
  ShorthandOtherDe
  WordDeSmall "(" WordDeSmall ")" WordDeSmall
  WordDeSmall "(" WordDeSmall ")"
  WordDeSmall "!"
  WordDeSmall "-" WordDeSmall "(" WordDeSmall ")"
  WordDeSmall "-" WordDeSmall
  WordDeSmall "-"
  WordDeSmall ":"
  WordDeSmall
  "-" WordDeSmall "(" WordDeSmall ")"
  "-" WordDeSmall
  "(" WordDeSmall ")" WordDeSmall "-"
  "(" WordDeSmall ")" WordDeSmall "(" WordDeSmall ")"
  "(" WordDeSmall ")" WordDeSmall
  "(" WordDeSmall "-" ")" WordDeSmall
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
pub fn word_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        alt((
            date_de_parser.recognize(),
            // beware: negative lookahead for ".", otherwise consumes part of higher-up UsageItem which then fails
            terminated(integer_parser, not('.')).recognize(),
            shorthand_variable_de_parser,
            shorthand_combination_de_parser,
            shorthand_de_parser,
            shorthand_other_de_parser,
            (
                word_de_small_parser,
                '(',
                word_de_small_parser,
                ')',
                word_de_small_parser,
            )
                .recognize(),
            (word_de_small_parser, '(', word_de_small_parser, ')').recognize(),
            (word_de_small_parser, '!').recognize(),
            (
                word_de_small_parser,
                '-',
                word_de_small_parser,
                '(',
                word_de_small_parser,
                ')',
            )
                .recognize(),
            (word_de_small_parser, '-', word_de_small_parser).recognize(),
            (word_de_small_parser, '-').recognize(),
        )),
        alt((
            (word_de_small_parser, ':').recognize(),
            word_de_small_parser,
            ('-', word_de_small_parser, '(', word_de_small_parser, ')').recognize(),
            ('-', word_de_small_parser).recognize(),
            ('(', word_de_small_parser, ')', word_de_small_parser, '-').recognize(),
            (
                '(',
                word_de_small_parser,
                ')',
                word_de_small_parser,
                '(',
                word_de_small_parser,
                ')',
            )
                .recognize(),
            ('(', word_de_small_parser, ')', word_de_small_parser).recognize(),
            ('(', word_de_small_parser, '-', ')', word_de_small_parser).recognize(),
            (word_de_big_parser, '-', word_de_big_parser).recognize(),
            (word_de_big_parser, '-', word_de_small_parser).recognize(),
            (word_de_big_parser, '-').recognize(),
            (word_de_big_parser, '(', word_de_small_parser, ')').recognize(),
            (word_de_big_parser, "...").recognize(),
            word_de_big_parser,
        )),
        alt((
            ('(', word_de_big_parser, '-', ')', word_de_big_parser).recognize(),
            (word_ka_small_parser, '-', word_de_big_parser).recognize(),
            (word_ka_small_parser, '!').recognize(),
            word_ka_small_parser,
            // ...
        )),
    ))
    .context(StrContext::Label("word_de"))
    .parse_next(input)
}

/*
ShorthandVariableDe
  "zs." "-" WordDeSmall
*/
pub fn shorthand_variable_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((separated_pair("zs.", '-', word_de_small_parser).recognize(),))
        .context(StrContext::Label("shorthand_variable_de"))
        .parse_next(input)
}

/*
ShorthandCombinationDe
  "a." ws Category
  "a." ws PartOfSpeech
  "imS" ":"
*/
pub fn shorthand_combination_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        separated_pair("a.", ws_parser, category_parser).recognize(),
        separated_pair("a.", ws_parser, part_of_speech_parser).recognize(),
        ("imS", ':').recognize(),
    ))
    .context(StrContext::Label("shorthand_combination_de"))
    .parse_next(input)
}

/*
ShorthandOtherDe
  "bzw."
  "ca."
  "d.h."
  "durch-ea."
  "georg."
  "griech."
  "kaukas."
  "NG"
  "od."
  "OG"
  "SG"
  "umg."
  "usw."
  "WG"
  "z.B."
*/
pub fn shorthand_other_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        "bzw.",
        "ca.",
        "d.h.",
        "durch-ea.",
        "georg.",
        "griech.",
        "kaukas.",
        "NG",
        "od.",
        "OG",
        "SG",
        "umg.",
        "usw.",
        "WG",
        "z.B.",
    ))
    .context(StrContext::Label("shorthand_other_de"))
    .parse_next(input)
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
pub fn shorthand_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        alt((
            "a.", "Abk.", "ag.", "b.", "Bed.", "d.O.", "do.", "DOZ", "e-e", "e-m", "e-n", "e-r",
            "e-s", "e.", "ea.", "ehm.", "et.", "Fn.", "FR", "fr.",
        )),
        alt((
            "g.", "gebr.", "Ggs.", "i.O.", "imS", "intr.", "IOZ", "Iter.", "j-d", "j-m", "j-n",
            "j-s", "L.", "l.", "m-e", "m-m", "m-n", "m-r", "m-s", "m.",
        )),
        alt((
            "m. Vn.", "mst", "neg.", "NG", "ng.", "NV", "Obj.", "od.", "OG", "og.", "OR", "OV",
            "OVZ", "P.", "Pkt.", "PR", "s.", "SG", "sg.", "sn",
        )),
        alt((
            "Subj.", "SupV", "SupVZ", "SV", "T.", "u. zw.", "u.", "v.", "Vn.", "w. Vn.", "WG",
            "wg.", "zs.",
        )),
    ))
    .context(StrContext::Label("shorthand_de"))
    .parse_next(input)
}

// note: don't validate day
/*
DateDe
  Integer "." "ws" MonthDe
*/
pub fn date_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    separated_pair(terminated(integer_parser, '.'), ws_parser, month_de_parser)
        .recognize()
        .context(StrContext::Label("date_de"))
        .parse_next(input)
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
pub fn month_de_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    alt((
        "Januar", "Februar", "März", "April", "Mai", "Juni", "Juli", "August", "Sept.", "Okt.",
        "Nov.", "Dez.",
    ))
    .context(StrContext::Label("month_de"))
    .parse_next(input)
}
