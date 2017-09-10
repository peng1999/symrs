extern crate symrs;

use symrs::symbolic::*;

#[test]
fn symbol_display() {
    let s = Symbol::new("x");
    assert_eq!(format!("{}", s), "x".to_string());
}

#[test]
fn construct_expr() {
    let int = Expr::integer(4);
    if let Expr::Integer(int) = int {
        assert_eq!(int, 4.into());
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
