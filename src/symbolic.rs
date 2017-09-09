use std::sync::Mutex;
use std::fmt::{self, Display};
use num::BigInt;
use symtern::prelude::*;
use symtern::{Pool, Sym};
use symtern::adaptors::{self, Inline};

use Expr::*;

#[derive(Debug)]
pub enum Expr<'a> {
    Integer(BigInt),
    Symbol(Symbol),
    Approximate(f64),

    Product(Vec<&'a Expr<'a>>),

    Undefined,
}

impl<'a> Expr<'a> {
    pub fn integer<I: Into<BigInt>>(i: I) -> Expr<'a> {
        Integer(i.into())
    }

    pub fn symbol(s: &str) -> Expr<'a> {
        Expr::Symbol(Symbol::new(s))
    }
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Integer(ref i) => write!(f, "{}", i),
            Expr::Symbol(ref s) => write!(f, "{}", s),
            _ => unreachable!(),
        }
    }
}

type InlinePool = Inline<Pool<str, u32>>;
type InlineSym = adaptors::InlineSym<Sym<u32>>;

lazy_static! {
    static ref SYMPOOL: Mutex<InlinePool> = Mutex::new(Inline::from(Pool::<str, u32>::new()));
}

/// The symbol in expressions such as x, y or delta.
#[derive(Debug, Clone, Copy)]
pub struct Symbol(InlineSym);

impl Symbol {
    /// Create a new symbol.
    pub fn new(s: &str) -> Symbol {
        let sym = SYMPOOL.lock().unwrap().intern(s).unwrap();
        Symbol(sym)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", SYMPOOL.lock().unwrap().resolve(&self.0).unwrap())
    }
}
