use nom::{error::ParseError, IResult, branch::alt, sequence::tuple, character::complete::char, combinator::recognize, bytes::complete::take_while1};

/*
WordKa
    WordKaHyphen
    WordKaSmall
*/
pub fn word_ka_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    alt((word_ka_hyphen_parser, word_ka_small_parser))(input)
}

/*
// note: allow only single hyphen in word
WordKaHyphen
    WordKaSmall "-" WordKaSmall
    // WordKaSmall "-" WordDeBig
*/
pub fn word_ka_hyphen_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    recognize(tuple((word_ka_small_parser, char('-'), word_ka_small_parser)))(input)
}

/*
// note: allow one letter
WordKaSmall
    CharKaSmall+
*/
pub fn word_ka_small_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    take_while1(is_char_ka)(input)
}

/*
CharKaSmall
    UNICODE_GEORGIAN_CHARACTER
*/
fn is_char_ka(c: char) -> bool {
    match c {
        '\u{10D0}'..='\u{10F0}' => true,
        _ => false,
    }
}
