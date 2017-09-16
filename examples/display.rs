extern crate symrs;

use symrs::Expr;

fn main() {
    let e = 1;
    let b = e;
    let x = Expr::symbol("x");
    let ec = x + e + b;
    let y = Expr::symbol("y");
    let eb = ec * y;

    println!("Repr: {:?}", eb);
    println!("Disp: {}", eb);
}
