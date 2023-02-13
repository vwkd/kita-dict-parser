use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, value},
    error::VerboseError,
    sequence::separated_pair,
    IResult,
};

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

pub fn part_of_speech_tag_parser(
    input: &str,
) -> IResult<&str, PartOfSpeechTag, VerboseError<&str>> {
    alt((
        map(part_of_speech_multiple_parser, PartOfSpeechTag::Multiple),
        map(part_of_speech_parser, PartOfSpeechTag::Single),
    ))(input)
}

/*
PartOfSpeechMultiple
  "S" ws "p.a."
  "S" ws "p.n."
*/

pub fn part_of_speech_multiple_parser(
    input: &str,
) -> IResult<&str, Vec<PartOfSpeech>, VerboseError<&str>> {
    map(
        separated_pair(
            value(PartOfSpeech::S, tag("S")),
            ws_parser,
            alt((
                value(PartOfSpeech::PA, tag("p.a.")),
                value(PartOfSpeech::PN, tag("p.n.")),
            )),
        ),
        |(pos1, pos2)| vec![pos1, pos2],
    )(input)
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

pub fn part_of_speech_parser(input: &str) -> IResult<&str, PartOfSpeech, VerboseError<&str>> {
    alt((
        alt((
            value(PartOfSpeech::A, tag("a")),
            value(PartOfSpeech::AD, tag("ad")),
            value(PartOfSpeech::ADDEM, tag("ad.dem.")),
            value(PartOfSpeech::ADINT, tag("ad.int.")),
            value(PartOfSpeech::ADREL, tag("ad.rel.")),
            value(PartOfSpeech::CJ, tag("cj")),
            value(PartOfSpeech::DEKL, tag("dekl")),
            value(PartOfSpeech::ENKL, tag("enkl")),
            value(PartOfSpeech::DRGR, tag("3.Gr.")),
            value(PartOfSpeech::INF, tag("inf")),
            value(PartOfSpeech::INT, tag("int")),
            value(PartOfSpeech::PA, tag("p.a.")),
            value(PartOfSpeech::PFP, tag("p.f.")),
            value(PartOfSpeech::PN, tag("p.n.")),
            value(PartOfSpeech::PP, tag("pp")),
            value(PartOfSpeech::PPP, tag("p.p.")),
            value(PartOfSpeech::PRDEM, tag("pr.dem.")),
            value(PartOfSpeech::PRINT, tag("pr.int.")),
            value(PartOfSpeech::PRPERS, tag("pr.pers.")),
            value(PartOfSpeech::PRPOSS, tag("pr.poss.")),
            value(PartOfSpeech::PRREL, tag("pr.rel.")),
        )),
        alt((
            value(PartOfSpeech::PRV, tag("prv")),
            value(PartOfSpeech::S, tag("S")),
            value(PartOfSpeech::SPN, tag("spn")),
        )),
    ))(input)
}