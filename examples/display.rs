extern crate symrs;

use symrs::sym::Symbol;

fn main() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");

    println!("eb : {}", 1 / (3 + x * y));
    println!("ed : {}", x - 5 + 3 / y);
}
