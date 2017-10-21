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
//! use symrs::sym::Expr;
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
//!
//! Sometimes rustc can't determine which impl to use when dealing with number literal,
//! in that case you should add a type specifier.
//!
//! ```
//! use symrs::sym::{Expr, Symbol};
//!
//! let x = Symbol::new("x");
//! // use `1i32` rather than `1`
//! let e = 1i32 + x;
//! ```


mod expr;
mod symbol;
mod impls;

pub use self::expr::Expr;
pub use self::symbol::Symbol;
