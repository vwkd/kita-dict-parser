use winnow::prelude::*;

/*
Temporality
  "†"
*/
#[derive(Debug, Clone)]
pub enum Temporality {
    Archaic,
}

pub fn temporality_parser<'a>(input: &mut &'a str) -> PResult<Temporality> {
    '†'.value(Temporality::Archaic).parse_next(input)
}
