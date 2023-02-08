use nom::{
    bytes::complete::take_while1,
    error::{context, VerboseError},
    IResult,
};

// note: allow one letter
/*
WordKaSmall
    CharKaSmall+
*/
pub fn word_ka_small_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context("word_ka_small", take_while1(is_char_ka))(input)
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
