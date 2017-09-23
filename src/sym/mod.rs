//! Definition of the expression representation.
//!
//! You can use operators directly:
//!
//! ```
//! use symrs::sym::{Expr, Symbol};
//!
//! let x = Symbol::new("x");
//! let b = x + 1;
//! assert_eq!(b, Expr::Sum(vec![Expr::Sym(x), Expr::integer(1)]));
//! ```

use std::fmt::{self, Display};
use num::BigInt;
use itertools::Itertools;

mod symbol;
mod impls;
pub use self::symbol::Symbol;

use Expr::*;

/// An expression representation.
#[derive(Debug, Clone, PartialEq)]
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

    /// Construct an approximate value.
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
