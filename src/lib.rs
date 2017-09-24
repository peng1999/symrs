//! A symbolic computation library.
//!
//! This library contains three modules: the symbolic engine, the Computer Algebra System(CAS), and
//! a parser/writer.
//! The symbolic module contains functionality of storing and manipulating the symbolic math
//! expressions.

#![cfg_attr(check_doc, deny(missing_docs))]

extern crate num;
extern crate symtern;
#[macro_use]
extern crate lazy_static;
extern crate itertools;

pub mod sym;

pub use sym::Expr;
