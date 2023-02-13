use nom::{
    bytes::complete::take_while1,
    character::complete::{char, satisfy},
    combinator::{opt, recognize},
    error::{context, VerboseError},
    sequence::{pair, tuple},
    IResult,
};

/*
WordDeSmall
    CharDeSmall+ ("'" "s"?)?
*/
pub fn word_de_small_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "word_de_small",
        recognize(pair(
            take_while1(is_char_de_small),
            opt(pair(char('\''), opt(char('s')))),
        )),
    )(input)
}

// note: require at least two letters
/*
WordDeBig
    CharDeBig CharDeSmall+
*/
pub fn word_de_big_parser(input: &str) -> IResult<&str, &str, VerboseError<&str>> {
    context(
        "word_de_big",
        recognize(tuple((
            satisfy(is_char_de_big),
            take_while1(is_char_de_small),
        ))),
    )(input)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_de_small_parser() {
        let a = word_de_small_parser("bär");
        assert!(a.is_ok());

        let b = word_de_small_parser(" bär");
        assert!(b.is_err());

        let c = word_de_small_parser("Bär");
        assert!(c.is_err());
    }

    #[test]
    fn test_word_de_big_parser() {
        let a = word_de_big_parser("Bär");
        assert!(a.is_ok());

        let b = word_de_big_parser(" Bär");
        assert!(b.is_err());

        let c = word_de_big_parser("bär");
        assert!(c.is_err());
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
}
