use std::sync::Mutex;
use num::BigInt;
use symtern::prelude::*;
use symtern::{Pool, Sym};
use symtern::adaptors::{Inline, self};

#[derive(Debug)]
pub enum Expr {
    Integer(BigInt),
    Symbol(Symbol),
    Float(f64),

    Product(Box<Expr>, Box<Expr>),
}

type InlinePool = Inline<Pool<str, u32>>;
type InlineSym = adaptors::InlineSym<Sym<u32>>;

lazy_static! {
    static ref sympool: Mutex<InlinePool> = Mutex::new(Inline::from(Pool::<str, u32>::new()));
}

#[derive(Debug, Clone, Copy)]
pub struct Symbol(InlineSym);

impl Symbol {
    pub fn new(s: &str) -> Symbol {
        let sym = sympool.lock().unwrap().intern(s).unwrap();
        Symbol(sym)
    }
}
