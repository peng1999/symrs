extern crate symrs;

use symrs::symbolic::*;

#[test]
fn symbol_display() {
    let s = Symbol::new("x");
    assert_eq!(format!("{}", s), "x".to_string());
}
