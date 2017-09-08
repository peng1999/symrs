//! A symbolic computation library.
//!
//! This library contains three modules: the symbolic engine, the Computer Algebra System(CAS), and
//! a parser/writer.

extern crate num;
extern crate symtern;
#[macro_use]
extern crate lazy_static;

pub mod symbolic;

pub use symbolic::Expr;
