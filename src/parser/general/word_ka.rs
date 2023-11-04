use winnow::error::StrContext;
use winnow::prelude::*;
use winnow::token::take_while;

// note: allow one letter
/*
WordKaSmall
    CharKaSmall+
*/
pub fn word_ka_small_parser<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., is_char_ka)
        .context(StrContext::Label("word_ka_small"))
        .parse_next(input)
}

/*
CharKaSmall
    UNICODE_GEORGIAN_CHARACTER
*/
pub fn is_char_ka(c: char) -> bool {
    match c {
        'ა'..='ჰ' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_ka_small_parser() {
        let a = word_ka_small_parser(&mut "კატა");
        assert!(a.is_ok());

        let b = word_ka_small_parser(&mut " კატა");
        assert!(b.is_err());

        // beware: first is capital letter, different from small letters!
        let c = word_ka_small_parser(&mut "Კატა");
        assert!(c.is_err());
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
}
