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
pub enum Expr {
    /// Represent an integer.
    Integer(BigInt),
    /// Represent a symbol like `x`, `y`, or `delta`.
    Sym(Symbol),
    /// Represent a float approximate value.
    Approx(f64),

    /// Represent a sum of some expressions, like `x + y + z`.
    Sum(Vec<Expr>),

    /// Represent a product of some expressions, like `x * y * z`.
    Product(Vec<Expr>),

    /// Represent an undefined value.
    Undefined,
}

/// Constructors.
impl Expr {
    /// Construct a integer value.
    pub fn integer<I: Into<BigInt>>(i: I) -> Expr {
        Integer(i.into())
    }

    /// Construct a symbol.
    pub fn symbol(s: &str) -> Expr {
        Sym(Symbol::new(s))
    }

    pub fn approximate<F: Into<f64>>(f: F) -> Expr {
        Approx(f.into())
    }
}

/// Classify a `Expr`.
impl Expr {
    pub fn is_primitive(&self) -> bool {
        match *self {
            Integer(_) | Sym(_) | Approx(_) => true,
            _ => false,
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_f = |elt: &Expr, f: &mut FnMut(&Display) -> fmt::Result| if elt.is_primitive() {
            f(elt)
        } else {
            f(&format_args!("({})", elt))
        };
        match *self {
            Integer(ref i) => write!(f, "{}", i),
            Sym(s) => write!(f, "{}", s),
            Approx(n) => write!(f, "{}", n),
            Sum(ref args) => write!(f, "{}", args.into_iter().format_with(" + ", fmt_f)),
            Product(ref args) => write!(f, "{}", args.into_iter().format_with(" * ", fmt_f)),
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
