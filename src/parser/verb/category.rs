use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::value,
    error::{context, VerboseError},
    IResult,
};

/*
VerbCategory
    "IV¹"
    "IV²"
    "IV³"
    "IV⁴"
    "KT"
    "MV"
    "P¹"
    "P²"
    "P³"
    "RM¹"
    "RM²"
    "RM³"
    "RM⁴"
    "RP¹"
    "RP²"
    "RP³"
    "RP⁴"
    "RP⁵"
    "RP⁶"
    "RP⁷"
    "T¹"
    "T²"
    "T³"
    "T⁴"
    "T⁵"
    "ZP¹"
    "ZP²"
    "ZP³"
*/
#[derive(Clone, Debug)]
pub enum VerbCategory {
    IV1,
    IV2,
    IV3,
    IV4,
    KT,
    MV,
    P1,
    P2,
    P3,
    RM1,
    RM2,
    RM3,
    RM4,
    RP1,
    RP2,
    RP3,
    RP4,
    RP5,
    RP6,
    RP7,
    T1,
    T2,
    T3,
    T4,
    T5,
    ZP1,
    ZP2,
    ZP3,
}

pub fn category_parser(input: &str) -> IResult<&str, VerbCategory, VerboseError<&str>> {
    context(
        "category",
        alt((
            alt((
                value(VerbCategory::IV1, tag("IV¹")),
                value(VerbCategory::IV2, tag("IV²")),
                value(VerbCategory::IV3, tag("IV³")),
                value(VerbCategory::IV4, tag("IV⁴")),
                value(VerbCategory::KT, tag("KT")),
                value(VerbCategory::MV, tag("MV")),
                value(VerbCategory::P1, tag("P¹")),
                value(VerbCategory::P2, tag("P²")),
                value(VerbCategory::P3, tag("P³")),
                value(VerbCategory::RM1, tag("RM¹")),
                value(VerbCategory::RM2, tag("RM²")),
                value(VerbCategory::RM3, tag("RM³")),
                value(VerbCategory::RM4, tag("RM⁴")),
            )),
            alt((
                value(VerbCategory::RP1, tag("RP¹")),
                value(VerbCategory::RP2, tag("RP²")),
                value(VerbCategory::RP3, tag("RP³")),
                value(VerbCategory::RP4, tag("RP⁴")),
                value(VerbCategory::RP5, tag("RP⁵")),
                value(VerbCategory::RP6, tag("RP⁶")),
                value(VerbCategory::RP7, tag("RP⁷")),
                value(VerbCategory::T1, tag("T¹")),
                value(VerbCategory::T2, tag("T²")),
                value(VerbCategory::T3, tag("T³")),
                value(VerbCategory::T4, tag("T⁴")),
                value(VerbCategory::T5, tag("T⁵")),
                value(VerbCategory::ZP1, tag("ZP¹")),
                value(VerbCategory::ZP2, tag("ZP²")),
                value(VerbCategory::ZP3, tag("ZP³")),
            )),
        )),
    )(input)
}
