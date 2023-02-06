use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::{map, recognize},
    sequence::{delimited, tuple},
    IResult,
};
use nom_supreme::error::ErrorTree;

use super::Value;

/*
HeadwordKa
    WordRootKa
    WordKa
*/
#[derive(Debug)]
pub enum HeadwordKa<'a> {
    Plain(&'a str),
    WithRoot(WordRootKa<'a>),
}

pub fn headword_ka_parser(input: &str) -> IResult<&str, HeadwordKa, ErrorTree<&str>> {
    alt((
        map(word_root_ka_parser, HeadwordKa::WithRoot),
        map(word_ka_parser, HeadwordKa::Plain),
    ))(input)
}

/*
WordRootKa
    RootKa WordKaSmall
    WordKaSmall RootKa WordKaSmall
    WordKaSmall RootKa
*/
#[derive(Debug)]
/// srtart, root, end
pub struct WordRootKa<'a>(Option<Value<'a>>, Value<'a>, Option<Value<'a>>);

pub fn word_root_ka_parser(input: &str) -> IResult<&str, WordRootKa, ErrorTree<&str>> {
    alt((
        map(
            tuple((root_ka_parser, word_ka_small_parser)),
            |(root, end)| WordRootKa(None, root, Some(end)),
        ),
        map(
            tuple((word_ka_small_parser, root_ka_parser, word_ka_small_parser)),
            |(start, root, end)| WordRootKa(Some(start), root, Some(end)),
        ),
        map(
            tuple((word_ka_small_parser, root_ka_parser)),
            |(start, root)| WordRootKa(Some(start), root, None),
        ),
    ))(input)
}

/*
RootKa
    "**" WordKa "**"
*/
pub fn root_ka_parser(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    delimited(tag("**"), word_ka_parser, tag("**"))(input)
}

/*
WordKa
    WordKaHyphen
    WordKaSmall
*/
pub fn word_ka_parser(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
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
pub fn word_ka_hyphen_parser(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
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
pub fn word_ka_small_parser(input: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    take_while1(is_char_ka)(input)
}

#[test]
fn test_word_ka_small_parser() {
    let a = word_ka_small_parser("კატა");
    assert!(a.is_ok());

    let b = word_ka_small_parser(" კატა");
    assert!(b.is_err());

    // beware: first is capital letter, different from small letters!
    let c = word_ka_small_parser("Კატა");
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
