//! A symbolic computation library.
//!
//! This library contains three modules: the symbolic engine, the Computer Algebra System(CAS), and
//! a parser/writer.
//! The symbolic module contains functionality of storing and manipulating the symbolic math
//! expressions.

#![cfg_attr(check_doc, deny(missing_docs))]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]

extern crate num;
extern crate symtern;
#[macro_use]
extern crate lazy_static;
extern crate itertools;

pub mod sym;

#[macro_use]
extern crate cfg_if;

cfg_if! { if #[cfg(feature = "parser")] {
    #[macro_use]
    extern crate nom;

    cfg_if! { if #[cfg(test)] {
        #[macro_use]
        extern crate nom_test_helpers;
        extern crate float_cmp;

        #[macro_use]
        mod test_util;
    }}

    pub mod par;
}}
