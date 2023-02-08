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
Categories
    "(" Category ("," ws Category)* ")"
*/
#[derive(Debug)]
pub struct Categories(Vec<Category>);

pub fn categories_parser(input: &str) -> IResult<&str, Categories, VerboseError<&str>> {
    context(
        "categories",
        map(
            delimited(
                char('('),
                separated_list1(terminated(char(','), ws_parser), category_parser),
                char(')'),
            ),
            Categories,
        ),
    )(input)
}

/*
Category
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
pub enum Category {
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

pub fn category_parser(input: &str) -> IResult<&str, Category, VerboseError<&str>> {
    context(
        "category",
        terminated(
            alt((
                alt((
                    value(Category::AN, tag("an")),
                    value(Category::ARCH, tag("arch")),
                    value(Category::ATSCH, tag("atsch")),
                    value(Category::BIOL, tag("biol")),
                    value(Category::BOT, tag("bot")),
                    value(Category::CHEM, tag("chem")),
                    value(Category::CHEW, tag("chew")),
                    value(Category::DESP, tag("desp")),
                    value(Category::DSCHAW, tag("dschaw")),
                    value(Category::ELEKTR, tag("elektr")),
                    value(Category::ETHN, tag("ethn")),
                    value(Category::EUPH, tag("euph")),
                    value(Category::FAM, tag("fam")),
                    value(Category::GEOL, tag("geol")),
                    value(Category::GR, tag("gr")),
                    value(Category::GUD, tag("gud")),
                    value(Category::GUR, tag("gur")),
                    value(Category::HIST, tag("hist")),
                    value(Category::IMER, tag("imer")),
                    value(Category::ING, tag("ing")),
                )),
                alt((
                    value(Category::IRO, tag("iro")),
                    value(Category::JUR, tag("jur")),
                    value(Category::KACH, tag("kach")),
                    value(Category::KHAR, tag("khar")),
                    value(Category::KHIS, tag("khis")),
                    value(Category::KIND, tag("Kind")),
                    value(Category::KOLL, tag("koll")),
                    value(Category::LANDW, tag("landw")),
                    value(Category::LETSCH, tag("letsch")),
                    value(Category::MATH, tag("math")),
                    value(Category::MED, tag("med")),
                    value(Category::MIL, tag("mil")),
                    value(Category::MIN, tag("min")),
                    value(Category::MINGR, tag("mingr")),
                    value(Category::MOCH, tag("moch")),
                    value(Category::MORAL, tag("moral")),
                    value(Category::MTHIUL, tag("mthiul")),
                    value(Category::MUS, tag("mus")),
                    value(Category::MYTH, tag("myth")),
                    value(Category::NZ, tag("nz")),
                    value(Category::OIMER, tag("o-imer")),
                )),
                alt((
                    value(Category::ORATSCH, tag("o-ratsch")),
                    value(Category::PHOTOGR, tag("photogr")),
                    value(Category::PHYS, tag("phys")),
                    value(Category::POET, tag("poet")),
                    value(Category::POL, tag("pol")),
                    value(Category::PSCH, tag("psch")),
                    value(Category::RATSCH, tag("ratsch")),
                    value(Category::RL, tag("rl")),
                    value(Category::SPO, tag("spo")),
                    value(Category::TECH, tag("tech")),
                    value(Category::THIAN, tag("thian")),
                    value(Category::THUSCH, tag("thusch")),
                    value(Category::TSCHAN, tag("tschan")),
                    value(Category::TYP, tag("typ")),
                    value(Category::UIMER, tag("u-imer")),
                    value(Category::UMG, tag("umg")),
                    value(Category::UNK, tag("unk")),
                    value(Category::URATSCH, tag("u-ratsch")),
                    value(Category::VR, tag("vr")),
                    value(Category::VULG, tag("vulg")),
                    value(Category::ZO, tag("zo")),
                )),
            )),
            char('.'),
        ),
    )(input)
}
