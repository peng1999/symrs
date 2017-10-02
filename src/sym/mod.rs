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


mod expr;
mod symbol;
mod impls;

pub use self::expr::Expr;
pub use self::symbol::Symbol;
