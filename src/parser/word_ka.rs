use nom::{
    branch::alt, bytes::complete::take_while1, character::complete::char, combinator::recognize,
    error::ParseError, sequence::tuple, IResult,
};

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

#[test]
fn test_word_ka_small_parser() {
    let a = word_ka_small_parser::<nom::error::Error<&str>>("კატა");
    assert!(a.is_ok());

    let b = word_ka_small_parser::<nom::error::Error<&str>>(" კატა");
    assert!(b.is_err());

    // beware: first is capital letter, different from small letters!
    let c = word_ka_small_parser::<nom::error::Error<&str>>("Კატა");
    assert!(c.is_err());
}

/*
CharKaSmall
    UNICODE_GEORGIAN_CHARACTER
*/
fn is_char_ka(c: char) -> bool {
    match c {
        'ა'..='ჰ' => true,
        _ => false,
    }
}

#[test]
fn test_is_char_ka() {
    assert!(is_char_ka('ა'));
    assert!(is_char_ka('ჯ'));
    assert!(is_char_ka('ჰ'));
    assert!(is_char_ka('პ'));
    assert!(is_char_ka('ყ'));
    assert!(is_char_ka('ტ'));
    assert!(is_char_ka('წ'));

    // beware: capital letters, different from small letters!
    assert!(!is_char_ka('Ა'));
    assert!(!is_char_ka('Ჯ'));
    assert!(!is_char_ka('Ჰ'));
    assert!(!is_char_ka('Პ'));
    assert!(!is_char_ka('Ყ'));
    assert!(!is_char_ka('Ტ'));
    assert!(!is_char_ka('Წ'));
}