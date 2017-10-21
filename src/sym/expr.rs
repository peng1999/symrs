use std::fmt::{self, Display};
use num::BigInt;
use itertools::Itertools;

use super::Symbol;
use self::Expr::*;

/// An owned expression representation.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Represent an integer.
    Integer(BigInt),
    /// Represent a symbol like `x`, `y`, or `delta`.
    Sym(Symbol),
    /// Represent a float approximate value.
    Approx(f64),

    /// Represent negative of a expression.
    Neg(Box<Expr>),

    /// Represent a sum of some expressions, like `x + y + z`.
    Sum(Vec<Expr>),
    /// Represent a product of some expressions, like `x * y * z`.
    Product(Vec<Expr>),
    /// Represent a ratio.
    Ratio(Box<Expr>, Box<Expr>),
    /// Represent a power.
    Pow(Box<Expr>, Box<Expr>),

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

    /// Construct negative of an expression.
    pub fn negative<E: Into<Expr>>(e: E) -> Expr {
        Neg(Box::new(e.into()))
    }

    /// Construct a exponentiation expression.
    pub fn pow<E: Into<Expr>>(self, e: E) -> Expr {
        Pow(Box::new(self), Box::new(e.into()))
    }
}

/// Classify a `Expr`.
impl Expr {
    /// Check if the expression is a primitive.
    /// # Examples
    /// ```
    /// use symrs::sym::{Expr, Symbol};
    ///
    /// assert_eq!(Expr::integer(1).is_primitive(), true);
    /// assert_eq!(Expr::approximate(1).is_primitive(), true);
    /// let x = Symbol::new("x");
    /// assert_eq!((x + 1).is_primitive(), false);
    /// ```
    pub fn is_primitive(&self) -> bool {
        match *self {
            Integer(_) | Sym(_) | Approx(_) => true,
            _ => false,
        }
    }

    /// Get the priority rank when they are displayed. A lower number implys a higher priority.
    pub fn priority_rank(&self) -> i32 {
        match *self {
            Integer(..) | Sym(..) | Approx(..) | Undefined => 0,
            Pow(..) => 2,
            Neg(..) => 3,
            Product(..) | Ratio(..) => 4,
            Sum(..) => 5,
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt_f = |elt: &Expr, f: &mut FnMut(&Display) -> fmt::Result| if elt.priority_rank() <
            self.priority_rank()
        {
            f(elt)
        } else {
            f(&format_args!("({})", elt))
        };

        match *self {
            Undefined => write!(f, "Undefined"),
            Integer(ref i) => write!(f, "{}", i),
            Sym(s) => write!(f, "{}", s),
            Approx(n) => write!(f, "{}", n),
            Neg(ref e) => fmt_f(e, &mut |e| write!(f, "-{}", e)),
            Sum(ref args) => write!(f, "{}", args.into_iter().format_with(" + ", fmt_f)),
            Product(ref args) => write!(f, "{}", args.into_iter().format_with(" * ", fmt_f)),
            Ratio(ref n, ref d) => {
                fmt_f(
                    n,
                    &mut |n_| fmt_f(d, &mut |d_| write!(f, "{} / {}", n_, d_)),
                )
            }
            Pow(ref e, ref p) => {
                fmt_f(
                    e,
                    &mut |e_| fmt_f(p, &mut |p_| write!(f, "{} ^ {}", e_, p_)),
                )
            },
        }
    }
}
