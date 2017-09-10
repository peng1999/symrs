//! Definition of the expression representation.

use std::sync::Mutex;
use std::fmt::{self, Display};
use num::BigInt;
use symtern::prelude::*;
use symtern::{Pool, Sym};
use symtern::adaptors::{self, Inline};
use itertools::Itertools;

use Expr::*;

/// An expression representation.
#[derive(Debug, Clone)]
pub enum Expr<'a> {
    /// Represent an integer.
    Integer(BigInt),
    /// Represent a symbol like `x`, `y`, or `delta`.
    Symbol(Symbol),
    /// Represent a float approximate value.
    Approximate(f64),

    /// Represent a sum of some expressions, like `x + y + z`.
    Sum(Vec<&'a Expr<'a>>),

    /// Represent a product of some expressions, like `x * y * z`.
    Product(Vec<&'a Expr<'a>>),

    /// Represent an undefined value.
    Undefined,
}

impl<'a> Expr<'a> {
    /// Construct a integer value.
    pub fn integer<I: Into<BigInt>>(i: I) -> Expr<'static> {
        Integer(i.into())
    }

    /// Construct a symbol.
    pub fn symbol(s: &str) -> Expr<'static> {
        Expr::Symbol(Symbol::new(s))
    }

    pub fn approximate<F: Into<f64>>(f: F) -> Expr<'static> {
        Expr::Approximate(f.into())
    }
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Integer(ref i) => write!(f, "{}", i),
            Expr::Symbol(s) => write!(f, "{}", s),
            Approximate(n) => write!(f, "{}", n),
            Sum(ref args) => {
                write!(f, "{}", args.into_iter().map(|a| format!("({})", a)).format("+"))
            },
            _ => unimplemented!(),
        }
    }
}

type InlinePool = Inline<Pool<str, u32>>;
type InlineSym = adaptors::InlineSym<Sym<u32>>;

lazy_static! {
    static ref SYMPOOL: Mutex<InlinePool> = Mutex::new(Inline::from(Pool::<str, u32>::new()));
}

/// The symbol type.
#[derive(Debug, Clone, Copy)]
pub struct Symbol(InlineSym);

impl Symbol {
    /// Construct a new symbol.
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
