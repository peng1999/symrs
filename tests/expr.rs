#![cfg_attr(feature = "cargo-clippy", allow(identity_op))]

extern crate symrs;
extern crate num;

use symrs::sym::*;
use num::BigInt;

#[test]
fn symbol_display() {
    let s = Symbol::new("x");
    assert_eq!(format!("{}", s), "x".to_string());
}

#[test]
fn construct_expr() {
    let int = Expr::integer(4);
    if let Expr::Integer(i) = int {
        assert_eq!(i, 4.into());
    } else {
        unreachable!();
    }
}

#[test]
fn display_expr() {
    let int = Expr::integer(4);
    assert_eq!(format!("{}", int), "4".to_string());

    let sym = Expr::symbol("x");
    assert_eq!(format!("{}", sym), "x".to_string());

    let app = Expr::approximate(1.32f64);
    assert_eq!(format!("{}", app), "1.32".to_string());
}

#[test]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
fn convert_from() {
    assert_eq!(Expr::from(3), Expr::integer(3));
    assert_eq!(Expr::from(3i8), Expr::integer(3));
    assert_eq!(
        Expr::from("1373663987".parse::<BigInt>().unwrap()),
        Expr::integer(1373663987)
    );

    assert_eq!(Expr::from(3.5554), Expr::Approx(3.5554));
    assert_eq!(Expr::from(3.5554f32), Expr::approximate(3.5554f32));
}

#[test]
fn operators_works() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    let z = Symbol::new("z");
    let big_3: BigInt = 3.into();

    // assert_eq!(x + y, Expr::Sum(vec![Expr::Sym(x), Expr::Sym(y)]));
    // assert_eq!(1 + y, Expr::Sum(vec![Expr::Integer(1.into()), Expr::Sym(y)]));
    // assert_eq!(x + 1, Expr::Sum(vec![Expr::Sym(x), Expr::Integer(1.into())]));

    // These operators should just works.
    let _ignore = [
        // Add
        x + y,          // sym + sym
        x + 1 + z,      // expr + sym
        x + (5 + y),    // sym + expr
        x + 4.0,        // sym + f
        4.0 + x,        // f + sym
        big_3.clone() + y,      // big + sym
        y + (x + big_3.clone()),// sym + big
        // Sub
        x - y,          // sym - sym
        x - 1 - z,      // expr - sym
        x - (5 - y),    // sym - expr
        x - 4.0,        // sym - f
        4.0 - x,        // f - sym
        big_3.clone() - y,      // big - sym
        y - (x - big_3.clone()),// sym - big
        // Mul
        x * y,          // sym * sym
        x * 1 * z,      // expr * sym
        x * (5 * y),    // sym * expr
        x * 4.0,        // sym * f
        4.0 * x,        // f * sym
        big_3.clone() * y,      // big * sym
        y * (x * big_3.clone()),// sym * big
        // Div
        x / y,          // sym / sym
        x / 1 / z,      // expr / sym
        x / (5 / y),    // sym / expr
        x / 4.0,        // sym / f
        4.0 / x,        // f / sym
        big_3.clone() / y,      // big / sym
        y / (x / big_3.clone()),// sym / big
        // Mixed
        x + (4 - (x / 2)) * z,
    ];
}
