use std::sync::Mutex;
use std::fmt::{self, Display};
use num::BigInt;
use symtern::prelude::*;
use symtern::{Pool, Sym};
use symtern::adaptors::{self, Inline};

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
    static ref SYMPOOL: Mutex<InlinePool> = Mutex::new(Inline::from(Pool::<str, u32>::new()));
}

/// The symbol in expressions such as x, y or delta.
#[derive(Debug, Clone, Copy)]
pub struct Symbol(InlineSym);

impl Symbol {
    /// Create a new symbol.
    /// # Examples
    /// ```
    /// use symrs::expr::Symbol;
    ///
    /// let s = Symbol::new("x");
    /// ```
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
