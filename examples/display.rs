extern crate symrs;

use symrs::Expr;

fn main() {
    let e = Expr::integer(1);
    let b = e.clone();
    let x = Expr::symbol("x");
    let eb = Expr::Sum(vec![&e, &b, &x]);
    println!("Repr: {:?}", eb);
    println!("Disp: {}", eb);
}