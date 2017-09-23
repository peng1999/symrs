use super::Expr;
use super::Expr::*;
use super::Symbol;
use num::{BigInt, BigUint};
use std::ops::{Add, Mul};

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


impl_op_for_t! {Add, add, Sum, Symbol}
impl_op_for_t! {Mul, mul, Product, Symbol}

macro_rules! impl_from {
    ($from_t: ty, $method: ident) => {
        impl From<$from_t> for Expr {
            fn from(x: $from_t) -> Expr {
                $crate::sym::Expr::$method(x)
            }
        }
    }
}

macro_rules! expand_conv_delegate {
    { (impl From<$from_t: ty> for Expr using $method: ident);* } => {
        (
            impl From<$from_t> for Expr {
                fn from(x: $from_t) -> Expr {
                    $crate::sym::Expr::$method(x)
                }
            }
        )*
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
