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
    assert_eq!(format!("{}", int), "4");

    let sym = Expr::symbol("x");
    assert_eq!(format!("{}", sym), "x");

    let app = Expr::approximate(1.32f64);
    assert_eq!(format!("{}", app), "1.32");

    let neg = - &sym;
    assert_eq!(format!("{}", neg), "- x");

    let plus = int + sym;
    assert_eq!(format!("{}", plus), "4 + x");

    let product = app * plus;
    assert_eq!(format!("{}", product), "1.32 * (4 + x)");
}

#[test]
fn equality() {
    let x = Symbol::new("x");

    assert_eq!(Expr::symbol("x"), x.into());
    assert_eq!(Expr::Sym(x), x.into());
    assert_ne!(Expr::symbol("y"), x.into());
    assert_eq!(Expr::integer(9), 9.into());
    assert_ne!(Expr::integer(9), 10.into());

    assert_eq!(x - 1, x - 1);
    assert_eq!(- x, - x);
}

#[test]
fn classify_expr() {
    let x = Symbol::new("x");

    // is_primitive works.
    assert_eq!(Expr::from(x).is_primitive(), true);
    assert_eq!(Expr::symbol("x").is_primitive(), true);
    assert_eq!(Expr::integer(3).is_primitive(), true);
    assert_eq!(Expr::approximate(3.5).is_primitive(), true);

    assert_eq!((x - 3).is_primitive(), false);
    assert_eq!((x + 3).is_primitive(), false);

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

// rustfmt can't process the aligned comments correctly, so let it skip the function.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[test]
fn operators_works() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");
    let z = Symbol::new("z");
    let big_3: BigInt = 3.into();

    assert_eq!(x + y, Expr::Sum(vec![Expr::Sym(x), Expr::Sym(y)]));
    assert_eq!(1 + y, Expr::Sum(vec![Expr::Integer(1.into()), Expr::Sym(y)]));
    assert_eq!(x + 1, Expr::Sum(vec![Expr::Sym(x), Expr::Integer(1.into())]));

    assert_eq!(
        2 * x * 5,
        Expr::Product(vec![
            Expr::integer(2),
            Expr::symbol("x"),
            Expr::integer(5),
        ]));

    // These operators should just works.
    let _ignore = [
        // Neg
        - x,            // - sym
        // Add
        x + y,          // sym + sym
        x + 1 + z,      // expr + sym
        x + (5 + y),    // sym + expr
        x + 4.0,        // sym + f
        4.0 + x,        // f + sym
        &big_3 + y,      // big + sym
        y + (x + &big_3),// sym + big
        // Sub
        x - y,          // sym - sym
        x - 1 - z,      // expr - sym
        x - (5 - y),    // sym - expr
        x - 4.0,        // sym - f
        4.0 - x,        // f - sym
        &big_3 - y,      // big - sym
        y - (x - &big_3),// sym - big
        // Mul
        x * y,          // sym * sym
        x * 1 * z,      // expr * sym
        x * (5 * y),    // sym * expr
        x * 4.0,        // sym * f
        4.0 * x,        // f * sym
        &big_3 * y,      // big * sym
        y * (x * &big_3),// sym * big
        // Div
        x / y,          // sym / sym
        x / 1 / z,      // expr / sym
        x / (5 / y),    // sym / expr
        x / 4.0,        // sym / f
        4.0 / x,        // f / sym
        &big_3 / y,      // big / sym
        y / (x / &big_3),// sym / big
        // Mixed
        x + (4 - (x / 2)) * - z,
    ];
}
