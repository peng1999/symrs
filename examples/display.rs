extern crate symrs;

use symrs::Expr;

fn main() {
    let e = 1;
    let b = e;
    let x = Expr::symbol("x");
    let ec = x + e + b;
    let y = Expr::symbol("y");
    let eb = ec * y;
    let ed = Expr::Ratio(Box::new(1.into()), Box::new(eb.clone()));

    println!("eb : {}", eb);
    println!("ed : {}", ed);
}
