//! Definition of the expression representation.
//!
//! # Creating a `Expr`
//!
//! `Expr` has implemented the `From` trait, so primitive values can be converted directly.
//! 
//! ```
//! use symrs::sym::{Expr, Symbol};
//!
//! let one = Expr::from(1);
//! let x: Expr = Symbol::new("x").into();
//! let about_half: Expr = 0.5f64.into();
//! ```
//!
//! Another way is to use a specified constructor.
//!
//! ```
//! use symrs::sym::{Expr, Symbol};
//!
//! let x = Expr::symbol("x");
//! let about_one = Expr::approximate(1);
//! let x_y = Expr::pow(x, about_one);
//! ```
//!
//! You can also use operators directly:
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
