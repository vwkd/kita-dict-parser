// todo: stricter chars
/*
CharKa
  GEORGIAN_LETTERS
  "-"
*/
pub fn is_char_ka(c: char) -> bool {
  match c {
      '\u{10D0}'..='\u{10F0}' | '-' => true,
      _ => false,
  }
}

// todo: stricter chars
/*
CharDe
GERMAN_LETTERS
DIGITS
ws
"-"
","
"("
")"
"."
*/
pub fn is_char_de(c: char) -> bool {
  is_alphanumeric(c)
      || is_umlaut(c)
      || is_space(c)
      || c == '-'
      || c == ','
      || c == '('
      || c == ')'
      || c == '.'
}

fn is_alphanumeric(c: char) -> bool {
  is_alphabetic(c) || is_digit(c)
}

fn is_alphabetic(c: char) -> bool {
  (c >= '\x41' && c <= '\x5A') || (c >= '\x61' && c <= '\x7A')
}

fn is_digit(c: char) -> bool {
  c >= '\x30' && c <= '\x39'
}

fn is_umlaut(c: char) -> bool {
  c == 'ä' || c == 'ö' || c == 'ü' || c == 'Ä' || c == 'Ö' || c == 'Ü' || c == 'ß' || c == 'ẞ'
}

fn is_space(c: char) -> bool {
  c == ' '
}
