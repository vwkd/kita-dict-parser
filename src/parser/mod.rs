use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::char;
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

mod chars;
use chars::{is_char_ka, is_char_de};
mod delimiters;
use delimiters::ws;

/*
Parser
  Entry EOF
*/
pub fn parser(input: &str) -> IResult<&str, Entry> {
    let (input, entry) = entry_parser(input)?;
    // todo: use nom::combinator::eof
    Ok((input, entry))
}

/*
Entry
  Term ws Meaning
*/
pub type Entry<'a> = (Term<'a>, Meaning<'a>);

//<'i, E: ParseError<&'i str>>
//, E
fn entry_parser(input: &str) -> IResult<&str, Entry> {
    separated_pair(term_parser, char(' '), meaning_parser)(input)
}

/*
Term
  CharKa+
*/
pub type Term<'a> = &'a str;

fn term_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Term, E> {
    take_while1(is_char_ka)(input)
}

// todo:
/*
Meaning
  Reference
  Usage+
*/
#[derive(Debug)]
pub enum Meaning<'a> {
    Usages(Vec<Usage<'a>>),
    Reference(Reference<'a>),
}

//<'i, E: ParseError<&'i str>>
//, E
fn meaning_parser(input: &str) -> IResult<&str, Meaning> {
    // match reference_parser(input) {
    //     Ok((s, r)) => Ok((s, Meaning::Reference(r))),
    //     Err(nom::Err::Error(e1)) => {
    //         match many1(usage_parser)(input) {
    //             Ok((s, u)) => Ok((s, Meaning::Usages(u))),
    //             // todo: return both errors
    //             Err(nom::Err::Error(e2)) => Err(nom::Err::Error(e1.or(e2))),
    //             Err(e) => Err(e)
    //         }
    //     },
    //     Err(e) => Err(e)
    // }
    alt((
        reference_parser.map(|r| Meaning::Reference(r)),
        many1(usage_parser).map(|u| Meaning::Usages(u)),
    ))(input)
}

// todo: add remaining
/*
Reference
  "s." ws Term
*/
#[derive(Debug)]
pub enum Reference<'a> {
    Direct(Term<'a>),
}

//<'i, E: ParseError<&'i str>> ....., E
fn reference_parser(input: &str) -> IResult<&str, Reference> {
    map(separated_pair(tag("s."), ws, term_parser), |(t, r)| {
        if t == "s." {
            Reference::Direct(r)
        } else {
            todo!()
        }
    })(input)
}

// todo: part of speech (pos)
/*
Usage
  Group (";" ws Group)*
*/
pub type Usage<'a> = Vec<Group<'a>>;

fn usage_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Usage, E> {
    separated_list1(tag("; "), group_parser)(input)
}

// semicolon separated
/*
Group
  Definition (";" ws Definition)*
*/
pub type Group<'a> = Vec<Definition<'a>>;

fn group_parser<'i, E: ParseError<&'i str>>(input: &'i str) -> IResult<&'i str, Group, E> {
    separated_list1(tag("; "), definition_parser)(input)
}

// todo: comma separated, after processing
/*
Definition
  CharDe+
*/
//type Definition<'a> = Vec<&'a str>;
pub type Definition<'a> = &'a str;

fn definition_parser<'i, E: ParseError<&'i str>>(
    input: &'i str,
) -> IResult<&'i str, Definition, E> {
    take_while1(is_char_de)(input)
    // many1(char_de_parser)(input)
}
