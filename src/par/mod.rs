use sym::Expr;
use num::BigInt;
use nom::{digit, float_s};
use std::str::{self, FromStr};

/// fn(&str)-> IResult<&str, Expr>
named!(integer<&str, Expr>,
       map!(
           map_res!(
               recognize!(pair!(
                   opt!(tag_s!("-")),
                   digit
               )),
               BigInt::from_str
           ),
           Expr::integer
       ));

/// fn(&str)-> IResult<&str, Expr>
named!(float<&str, Expr>,
       map!(
           float_s,
           Expr::approximate
       ));

#[cfg(test)]
mod test {
    use nom::IResult;
    use super::*;

    const _ASSERT_TYPE: &[fn(&str) -> IResult<&str, Expr>] = &[integer, float];

    #[test]
    fn integer_works() {
        assert_finished_and_eq!(integer("123"), Expr::integer(123));
        assert_finished_and_eq!(integer("-123"), Expr::integer(-123));
    }

    #[test]
    fn float_works() {
        assert_finished_and_near!(float("123.3"), 123.3);
        assert_finished_and_near!(float("123.3e3"), 123.3e3);
    }

}
