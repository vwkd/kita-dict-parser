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

#[test]
fn test_word_de_small_parser() {
    let a = word_de_small_parser::<nom::error::Error<&str>>("bär");
    assert!(a.is_ok());

    let b = word_de_small_parser::<nom::error::Error<&str>>(" bär");
    assert!(b.is_err());

    let c = word_de_small_parser::<nom::error::Error<&str>>("Bär");
    assert!(c.is_err());
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

#[test]
fn test_word_de_big_parser() {
    let a = word_de_big_parser::<nom::error::Error<&str>>("Bär");
    assert!(a.is_ok());

    let b = word_de_big_parser::<nom::error::Error<&str>>(" Bär");
    assert!(b.is_err());

    let c = word_de_big_parser::<nom::error::Error<&str>>("bär");
    assert!(c.is_err());
}

/*
CharDeSmall
    UNICODE_GERMAN_SMALL_CHARACTER
*/
fn is_char_de_small(c: char) -> bool {
    match c {
        'a'..='z' => true,
        'ä' => true,
        'ö' => true,
        'ü' => true,
        'ß' => true,
        _ => false,
    }
}

#[test]
fn test_is_char_de_small() {
    assert!(is_char_de_small('a'));
    assert!(is_char_de_small('j'));
    assert!(is_char_de_small('z'));
    assert!(is_char_de_small('ä'));
    assert!(is_char_de_small('ö'));
    assert!(is_char_de_small('ü'));
    assert!(is_char_de_small('ß'));

    assert!(!is_char_de_small('A'));
    assert!(!is_char_de_small('J'));
    assert!(!is_char_de_small('Z'));
    assert!(!is_char_de_small('Ä'));
    assert!(!is_char_de_small('Ö'));
    assert!(!is_char_de_small('Ü'));
    assert!(!is_char_de_small('ẞ'));
}

/*
CharDeBig
    UNICODE_GERMAN_BIG_CHARACTER
*/
fn is_char_de_big(c: char) -> bool {
    match c {
        'A'..='Z' => true,
        'Ä' => true,
        'Ö' => true,
        'Ü' => true,
        'ẞ' => true,
        _ => false,
    }
}

#[test]
fn test_is_char_de_big() {
    assert!(is_char_de_big('A'));
    assert!(is_char_de_big('J'));
    assert!(is_char_de_big('Z'));
    assert!(is_char_de_big('Ä'));
    assert!(is_char_de_big('Ö'));
    assert!(is_char_de_big('Ü'));
    assert!(is_char_de_big('ẞ'));

    assert!(!is_char_de_big('a'));
    assert!(!is_char_de_big('j'));
    assert!(!is_char_de_big('z'));
    assert!(!is_char_de_big('ä'));
    assert!(!is_char_de_big('ö'));
    assert!(!is_char_de_big('ü'));
    assert!(!is_char_de_big('ß'));
}