use winnow::combinator::{alt, repeat, terminated};
use winnow::error::StrContext;
use winnow::prelude::*;
use winnow::token::{take_till1, take_while};

use super::character::ws_parser;
use super::word_ka::is_char_ka;

const TERM_CHARS: [char; 19] = [
    '|', '-', '(', ')', '*', '?', '!', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹', '~', ',', '.',
];

const TERM_DELIMITERS: [char; 2] = [':', ';'];

/*
Term
  ???
*/
#[derive(Debug)]
pub struct Term<'a>(pub &'a str);

pub fn term_parser<'a>(input: &mut &'a str) -> PResult<Term<'a>> {
    alt((
        repeat::<_, _, (), _, _>(
            1..,
            terminated(
                take_while(1.., |c| (is_char_ka(c) || TERM_CHARS.contains(&c))),
                ws_parser,
            ),
        )
        .recognize(),
        take_till1(TERM_DELIMITERS).recognize(),
    ))
    .map(Term)
    .context(StrContext::Label("term"))
    .parse_next(input)
}
