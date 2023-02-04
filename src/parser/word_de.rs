use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{char, satisfy},
    combinator::recognize,
    error::ParseError,
    sequence::tuple,
    IResult,
};

/*
WordDe
    WordDeBig
    WordDeSmall
    WordDeHyphen
*/
pub fn word_de_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    alt((word_de_big_parser, word_de_small_parser, word_de_hyphen_parser))(input)
}

// todo: allow WordDeSmall?
/*
WordDeHyphen
    WordDeBig "-" WordDeBig
*/
pub fn word_de_hyphen_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    recognize(tuple((word_de_big_parser, char('-'), word_de_big_parser)))(input)
}

/*
// note: require at least two letters
WordDeSmall
    CharDeSmall{2,}
*/
pub fn word_de_small_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    recognize(tuple((
        satisfy(is_char_de_small),
        take_while1(is_char_de_small),
    )))(input)
}

/*
// note: require at least two letters
WordDeBig
    CharDeBig CharDeSmall+
*/
pub fn word_de_big_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    recognize(tuple((
        satisfy(is_char_de_big),
        take_while1(is_char_de_small),
    )))(input)
}

/*
CharDeSmall
    UNICODE_GERMAN_SMALL_CHARACTER
*/
fn is_char_de_small(c: char) -> bool {
    match c {
        '\u{10D0}'..='\u{10F0}' => true,
        'ä' => true,
        'ö' => true,
        'ü' => true,
        'ß' => true,
        _ => false,
    }
}

/*
CharDeBig
    UNICODE_GERMAN_BIG_CHARACTER
*/
fn is_char_de_big(c: char) -> bool {
    match c {
        '\u{10D0}'..='\u{10F0}' => true,
        'Ä' => true,
        'Ö' => true,
        'Ü' => true,
        'ẞ' => true,
        _ => false,
    }
}
