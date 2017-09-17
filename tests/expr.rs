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