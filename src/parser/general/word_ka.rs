use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::{map, recognize},
    error::ParseError,
    sequence::{delimited, tuple},
    IResult,
};

use super::Value;

/*
HeadwordKa
    WordRootKa
    WordKa
*/
pub fn headword_ka_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, WordRootKa<'i>, E> {
    alt((
        word_root_ka_parser,
        map(word_ka_parser, |value| WordRootKa(value.to_owned(), None)),
    ))(input)
}

/*
WordRootKa
    RootKa WordKaSmall
    WordKaSmall RootKa WordKaSmall
    WordKaSmall RootKa
*/
// note: needs owned string because can't concatenate string slices for word parts, e.g. "ა", "ბუშტ", "ული" of "ა**ბუშტ**ული"
#[derive(Debug)]
pub struct WordRootKa<'a>(String, Option<Value<'a>>);

pub fn word_root_ka_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, WordRootKa<'i>, E> {
    alt((
        map(
            tuple((root_ka_parser, word_ka_small_parser)),
            |(root, end)| WordRootKa(format!("{}{}", root, end), Some(root)),
        ),
        map(
            tuple((word_ka_small_parser, root_ka_parser, word_ka_small_parser)),
            |(start, root, end)| WordRootKa(format!("{}{}{}", start, root, end), Some(root)),
        ),
        map(
            tuple((word_ka_small_parser, root_ka_parser)),
            |(start, root)| WordRootKa(format!("{}{}", start, root), Some(root)),
        ),
    ))(input)
}

/*
RootKa
    "**" WordKa "**"
*/
pub fn root_ka_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, &'i str, E> {
    delimited(tag("**"), word_ka_parser, tag("**"))(input)
}

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
    "-" WordKaSmall
    WordKaSmall "-" WordKaSmall
    WordKaSmall "-"
    // WordKaSmall "-" WordDeBig
*/
pub fn word_ka_hyphen_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, &'i str, E> {
    alt((
        recognize(tuple((char('-'), word_ka_small_parser))),
        recognize(tuple((
            word_ka_small_parser,
            char('-'),
            word_ka_small_parser,
        ))),
        recognize(tuple((word_ka_small_parser, char('-')))),
    ))(input)
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
