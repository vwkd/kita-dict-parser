use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, value},
    error::{context, VerboseError},
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

use super::character::ws_parser;

/*
Tags
    "(" Tag ("," ws Tag)* ")"
*/
#[derive(Debug)]
pub struct Tags(Vec<Tag>);

pub fn tags_parser(input: &str) -> IResult<&str, Tags, VerboseError<&str>> {
    context(
        "tags",
        map(
            delimited(
                char('('),
                separated_list1(terminated(char(','), ws_parser), tag_parser),
                char(')'),
            ),
            Tags,
        ),
    )(input)
}

/*
Tag
    an.
    arch.
    atsch.
    biol.
    bot.
    chem.
    chew.
    desp.
    dschaw.
    elektr.
    ethn.
    euph.
    fam.
    geol.
    gr.
    gud.
    gur.
    hist.
    imer.
    ing.
    iro.
    jur.
    kach.
    khar.
    khis.
    Kind.
    koll.
    landw.
    letsch.
    math.
    med.
    mil.
    min.
    mingr.
    moch.
    moral.
    mthiul.
    mus.
    myth.
    nz.
    o-imer.
    o-ratsch.
    photogr.
    phys.
    poet.
    pol.
    psch.
    ratsch.
    rl.
    spo.
    tech.
    thian.
    thusch.
    tschan.
    typ.
    u-imer.
    umg.
    unk.
    u-ratsch.
    vr.
    vulg.
    zo.
*/
#[derive(Debug, Clone)]
pub enum Tag {
    AN,
    ARCH,
    ATSCH,
    BIOL,
    BOT,
    CHEM,
    CHEW,
    DESP,
    DSCHAW,
    ELEKTR,
    ETHN,
    EUPH,
    FAM,
    GEOL,
    GR,
    GUD,
    GUR,
    HIST,
    IMER,
    ING,
    IRO,
    JUR,
    KACH,
    KHAR,
    KHIS,
    KIND,
    KOLL,
    LANDW,
    LETSCH,
    MATH,
    MED,
    MIL,
    MIN,
    MINGR,
    MOCH,
    MORAL,
    MTHIUL,
    MUS,
    MYTH,
    NZ,
    OIMER,
    ORATSCH,
    PHOTOGR,
    PHYS,
    POET,
    POL,
    PSCH,
    RATSCH,
    RL,
    SPO,
    TECH,
    THIAN,
    THUSCH,
    TSCHAN,
    TYP,
    UIMER,
    UMG,
    UNK,
    URATSCH,
    VR,
    VULG,
    ZO,
}

pub fn tag_parser(input: &str) -> IResult<&str, Tag, VerboseError<&str>> {
    context(
        "tag",
        terminated(
            alt((
                alt((
                    value(Tag::AN, tag("an")),
                    value(Tag::ARCH, tag("arch")),
                    value(Tag::ATSCH, tag("atsch")),
                    value(Tag::BIOL, tag("biol")),
                    value(Tag::BOT, tag("bot")),
                    value(Tag::CHEM, tag("chem")),
                    value(Tag::CHEW, tag("chew")),
                    value(Tag::DESP, tag("desp")),
                    value(Tag::DSCHAW, tag("dschaw")),
                    value(Tag::ELEKTR, tag("elektr")),
                    value(Tag::ETHN, tag("ethn")),
                    value(Tag::EUPH, tag("euph")),
                    value(Tag::FAM, tag("fam")),
                    value(Tag::GEOL, tag("geol")),
                    value(Tag::GR, tag("gr")),
                    value(Tag::GUD, tag("gud")),
                    value(Tag::GUR, tag("gur")),
                    value(Tag::HIST, tag("hist")),
                    value(Tag::IMER, tag("imer")),
                    value(Tag::ING, tag("ing")),
                )),
                alt((
                    value(Tag::IRO, tag("iro")),
                    value(Tag::JUR, tag("jur")),
                    value(Tag::KACH, tag("kach")),
                    value(Tag::KHAR, tag("khar")),
                    value(Tag::KHIS, tag("khis")),
                    value(Tag::KIND, tag("Kind")),
                    value(Tag::KOLL, tag("koll")),
                    value(Tag::LANDW, tag("landw")),
                    value(Tag::LETSCH, tag("letsch")),
                    value(Tag::MATH, tag("math")),
                    value(Tag::MED, tag("med")),
                    value(Tag::MIL, tag("mil")),
                    value(Tag::MIN, tag("min")),
                    value(Tag::MINGR, tag("mingr")),
                    value(Tag::MOCH, tag("moch")),
                    value(Tag::MORAL, tag("moral")),
                    value(Tag::MTHIUL, tag("mthiul")),
                    value(Tag::MUS, tag("mus")),
                    value(Tag::MYTH, tag("myth")),
                    value(Tag::NZ, tag("nz")),
                    value(Tag::OIMER, tag("o-imer")),
                )),
                alt((
                    value(Tag::ORATSCH, tag("o-ratsch")),
                    value(Tag::PHOTOGR, tag("photogr")),
                    value(Tag::PHYS, tag("phys")),
                    value(Tag::POET, tag("poet")),
                    value(Tag::POL, tag("pol")),
                    value(Tag::PSCH, tag("psch")),
                    value(Tag::RATSCH, tag("ratsch")),
                    value(Tag::RL, tag("rl")),
                    value(Tag::SPO, tag("spo")),
                    value(Tag::TECH, tag("tech")),
                    value(Tag::THIAN, tag("thian")),
                    value(Tag::THUSCH, tag("thusch")),
                    value(Tag::TSCHAN, tag("tschan")),
                    value(Tag::TYP, tag("typ")),
                    value(Tag::UIMER, tag("u-imer")),
                    value(Tag::UMG, tag("umg")),
                    value(Tag::UNK, tag("unk")),
                    value(Tag::URATSCH, tag("u-ratsch")),
                    value(Tag::VR, tag("vr")),
                    value(Tag::VULG, tag("vulg")),
                    value(Tag::ZO, tag("zo")),
                )),
            )),
            char('.'),
        ),
    )(input)
}
