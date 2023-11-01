use winnow::combinator::{alt, separated_pair};
use winnow::prelude::*;

use super::character::ws_parser;

/*
PartOfSpeechTag
  PartOfSpeechMultiple
  PartOfSpeech
*/
#[derive(Debug, Clone)]
pub enum PartOfSpeechTag {
    Single(PartOfSpeech),
    Multiple(Vec<PartOfSpeech>),
}

pub fn part_of_speech_tag_parser<'a>(input: &mut &'a str) -> PResult<PartOfSpeechTag> {
    alt((
        part_of_speech_multiple_parser.map(PartOfSpeechTag::Multiple),
        part_of_speech_parser.map(PartOfSpeechTag::Single),
    ))
    .parse_next(input)
}

/*
PartOfSpeechMultiple
  "S" ws "p.a."
  "S" ws "p.n."
*/

pub fn part_of_speech_multiple_parser<'a>(input: &mut &'a str) -> PResult<Vec<PartOfSpeech>> {
    separated_pair(
        "S".value(PartOfSpeech::S),
        ws_parser,
        alt((
            "p.a.".value(PartOfSpeech::PA),
            "p.n.".value(PartOfSpeech::PN),
        )),
    )
    .map(|(pos1, pos2)| vec![pos1, pos2])
    .parse_next(input)
}

/*
PartOfSpeech
  "a"
  "ad"
  "ad.dem."
  "ad.int."
  "ad.rel."
  "cj"
  "dekl"
  "enkl"
  "3.Gr."
  "inf"
  "int"
  "p.a."
  "p.f."
  "p.n."
  "pp"
  "p.p."
  "pr.dem."
  "pr.int."
  "pr.pers."
  "pr.poss."
  "pr.rel."
  "prv"
  "S"
  "spn"

  // "A"
  // "aor"
  // "attr"
  // "cd"
  // "cj.pr."
  // "cj.f."
  // "cj.pt."
  // "comp"
  // "D/A"
  // "dim"
  // "E"
  // "f"
  // "fig"
  // "f/pl"
  // "fut"
  // "G"
  // "HV"
  // "I"
  // "imp"
  // "impf"
  // "(impf.)"
  // "m"
  // "m/pl"
  // "N"
  // "n"
  // "n/pl"
  // "opt"
  // "(perf.)"
  // "pf"
  // "pl"
  // "pl-pf"
  // "pr"
  // "sg"
  // "sub"
  // "sup"
  // "V"
*/
#[derive(Debug, Clone)]
pub enum PartOfSpeech {
    A,
    AD,
    ADDEM,
    ADINT,
    ADREL,
    CJ,
    DEKL,
    ENKL,
    DRGR,
    INF,
    INT,
    PA,
    PFP,
    PN,
    PP,
    PPP,
    PRDEM,
    PRINT,
    PRPERS,
    PRPOSS,
    PRREL,
    PRV,
    S,
    SPN,
}

pub fn part_of_speech_parser<'a>(input: &mut &'a str) -> PResult<PartOfSpeech> {
    alt((
        alt((
            "a".value(PartOfSpeech::A),
            "ad".value(PartOfSpeech::AD),
            "ad.dem.".value(PartOfSpeech::ADDEM),
            "ad.int.".value(PartOfSpeech::ADINT),
            "ad.rel.".value(PartOfSpeech::ADREL),
            "cj".value(PartOfSpeech::CJ),
            "dekl".value(PartOfSpeech::DEKL),
            "enkl".value(PartOfSpeech::ENKL),
            "3.Gr.".value(PartOfSpeech::DRGR),
            "inf".value(PartOfSpeech::INF),
            "int".value(PartOfSpeech::INT),
            "p.a.".value(PartOfSpeech::PA),
            "p.f.".value(PartOfSpeech::PFP),
            "p.n.".value(PartOfSpeech::PN),
            "pp".value(PartOfSpeech::PP),
            "p.p.".value(PartOfSpeech::PPP),
            "pr.dem.".value(PartOfSpeech::PRDEM),
            "pr.int.".value(PartOfSpeech::PRINT),
            "pr.pers.".value(PartOfSpeech::PRPERS),
            "pr.poss.".value(PartOfSpeech::PRPOSS),
            "pr.rel.".value(PartOfSpeech::PRREL),
        )),
        alt((
            "prv".value(PartOfSpeech::PRV),
            "S".value(PartOfSpeech::S),
            "spn".value(PartOfSpeech::SPN),
        )),
    ))
    .parse_next(input)
}
