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
use std::ops::{Add, Mul};
use num::BigInt;
use itertools::Itertools;

mod symbol;
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

impl<E: Into<Expr>> Add<E> for Expr {
    type Output = Expr;
    fn add(self, rhs: E) -> Self::Output {
        Sum(vec![self, rhs.into()])
    }
}

impl<E: Into<Expr>> Mul<E> for Expr {
    type Output = Expr;
    fn mul(self, rhs: E) -> Self::Output {
        Product(vec![self, rhs.into()])
    }
}

macro_rules! impl_op_for_t {
    ($trait:ident, $op:ident, $cons:ident, $lhs_t:ty) => {
        impl<T: Into<Expr>> $trait<T> for Symbol {
            type Output = Expr;
            fn $op(self, rhs: T) -> Self::Output {
                $cons(vec![self.into(), rhs.into()])
            }
        }
    }
}

impl_op_for_t! {Add, add, Sum, Symbol}
impl_op_for_t! {Mul, mul, Product, Symbol}

mod conv {
    use super::Expr;
    use super::Symbol;
    use num::{BigInt, BigUint};

    macro_rules! impl_from {
        ($from_t: ty, $method: ident) => {
            impl From<$from_t> for Expr {
                fn from(x: $from_t) -> Expr {
                    $crate::sym::Expr::$method(x)
                }
            }
        }
    }

    impl_from! { i8, integer }
    impl_from! { i16, integer }
    impl_from! { i32, integer }
    impl_from! { i64, integer }
    impl_from! { u8, integer }
    impl_from! { u16, integer }
    impl_from! { u32, integer }
    impl_from! { u64, integer }
    impl_from! { isize, integer }
    impl_from! { usize, integer }
    impl_from! { BigInt, integer }
    impl_from! { BigUint, integer }

    impl From<Symbol> for Expr {
        fn from(s: Symbol) -> Expr {
            Expr::Sym(s)
        }
    }

    impl_from! { f32, approximate }
    impl_from! { f64, approximate }
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
