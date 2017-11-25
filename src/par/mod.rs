use sym::Expr;
use num::BigInt;
use nom::{digit, float_s, IResult};
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

const GREEK: &str = "\u{391}\u{392}\u{393}\u{394}\u{395}\u{396}\u{397}\u{398}\u{399}\u{39A}\u{39B}\u{39C}\u{39D}\u{39E}\u{39F}\u{3A0}\u{3A1}\u{3A3}\u{3A4}\u{3A5}\u{3A6}\u{3A7}\u{3A8}\u{3A9}\u{3B1}\u{3B2}\u{3B3}\u{3B4}\u{3B5}\u{3B6}\u{3B7}\u{3B8}\u{3B9}\u{3BA}\u{3BB}\u{3BC}\u{3BD}\u{3BE}\u{3BF}\u{3C0}\u{3C1}\u{3C2}\u{3C3}\u{3C4}\u{3C5}\u{3C6}\u{3C7}\u{3C8}\u{3C9}";

const NUM: &str = "0123456789";
const IDENT: &str = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

named!(alphabet<&str, char>,
       alt!(
           one_of!(IDENT) |
           one_of!(GREEK)
           ));

named!(alphabet_or_num<&str, char>,
       alt!(
           alphabet |
           one_of!(NUM)
           ));

/// fn(&str)-> IResult<&str, Expr>
named!(symbol<&str, Expr>,
       map!(
           recognize!(pair!(
               alphabet,
               many0!(alphabet_or_num)
           )),
           Expr::symbol
       ));

named!(primitive<&str, Expr>,
       alt_complete!(
           float |
           integer |
           symbol
       ));

named!(parens<&str, Expr>,
       ws!(delimited!(
               tag_s!("("),
               parse_expr,
               tag_s!(")")
            ))
       );

named!(unit<&str, Expr>,
       alt_complete!(
           ws!(primitive) |
           parens
       ));

named!(pow<&str, Expr>,
       ws!(alt_complete!(
           do_parse!(
               left: unit >>
               tag_s!("^") >>
               right: negative >>
               (Expr::Pow(Box::new(left), Box::new(right)))) |
           unit
       )));

named!(negative<&str, Expr>,
       ws!(alt_complete!(
           pow |
           do_parse!(
               tag_s!("-") >> e: pow >>
               (Expr::Neg(Box::new(e)))
           )
       )));

named!(product<&str, Expr>, do_parse!(
       init: negative >>
       res: fold_many0!(
           pair!(one_of!("*/"),
                 negative),
           init,
           |acc, (op, val)| {
               match op {
                   '*' => match acc {
                       Expr::Product(mut f) => {
                           f.push(val);
                           Expr::Product(f)
                       },
                       e => e * val,
                   },
                   // must be '/'
                   _ => acc / val,
               }
           }) >>
       (res)
   ));

named!(sum<&str, Expr>, do_parse!(
       init: product >>
       res: fold_many0!(
           pair!(one_of!("+-"),
                 product),
           init,
           |acc, (op, val)| {
               let val: Expr = if op == '+' { val } else { - val };
               match acc {
                   Expr::Sum(mut f) => {
                       f.push(val);
                       Expr::Sum(f)
                   },
                   e => e + val,
               }
           }) >>
       (res)
   ));

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    sum(input)
}

#[cfg(test)]
mod test;
