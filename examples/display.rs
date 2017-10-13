extern crate symrs;

use symrs::sym::Symbol;

fn main() {
    let x = Symbol::new("x");
    let y = Symbol::new("y");

    println!("{}", 1 / (3 + x * y));
    println!("{}", x - (2u32 - x).pow(x) + 3 / y);
}
