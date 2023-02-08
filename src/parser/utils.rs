use nom::{
    error::{ErrorKind, ParseError},
    Err, IResult, InputLength, Parser,
};

#[derive(Debug)]
pub enum KitaError {
    IncreasingUsagesList,
}

/// like [`separated_list1`]
/// todo: replace with multi-range `separated_list` once implemented [nom#1613](https://github.com/rust-bakery/nom/issues/1613#issuecomment-1386162101)
pub fn separated_list2<I, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    move |mut i: I| {
        let mut res = Vec::new();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        // Parse the second element
        match sep.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i2, _)) => match f.parse(i2.clone()) {
                Err(e) => return Err(e),
                Ok((i2, o)) => {
                    res.push(o);
                    i = i2;
                }
            },
        }

        loop {
            let len = i.input_len();
            match sep.parse(i.clone()) {
                Err(Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    // infinite loop check: the parser must always consume
                    if i1.input_len() == len {
                        return Err(Err::Error(E::from_error_kind(i1, ErrorKind::SeparatedList)));
                    }

                    match f.parse(i1.clone()) {
                        Err(Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}
