extern crate symrs;

use symrs::Expr;

fn main() {
    let e = Expr::integer(1);
    let b = e.clone();
    let x = Expr::symbol("x");
    let ec = Expr::Sum(vec![e, b, x]);
    let y = Expr::symbol("y");
    let eb = Expr::Sum(vec![ec, y]);

    println!("Repr: {:?}", eb);
    println!("Disp: {}", eb);
}
