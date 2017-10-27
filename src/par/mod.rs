use sym::Expr;
use num::BigInt;
use nom::digit;
use std::str::{self, FromStr};

/// fn(&str)-> IResult<&str, Expr>
named!(integer<&str, Expr>,
       map!(
           map_res!(
               digit,
               BigInt::from_str
           ),
           Expr::integer
       ));

#[cfg(test)]
mod test {
    use nom::IResult;
    use super::*;

    const _INTEGER: fn(&str)-> IResult<&str, Expr> = integer;

    #[test]
    fn integer_works() {
        assert_done_and_eq!(integer("123"), Expr::integer(123));
    }

}
