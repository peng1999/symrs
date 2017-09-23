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
}

macro_rules! expand_op_delegate {
    {
        $(impl<T: Into<_>> $op_t:ident<T> for $expr:ident { use $cons:ident as $op:ident; } )*
    } => {
    }
}

impl_op_for_t! {Add, add, Sum, Symbol}
impl_op_for_t! {Mul, mul, Product, Symbol}

expand_op_delegate! {
    impl<T:Into<_>> Mul<T> for Expr { use Product as mul; }
}

macro_rules! expand_conv_delegate {
    { $(impl From<$from_t:ty> for Expr using $method:ident;)* } => {
        $(
            impl From<$from_t> for Expr {
                fn from(x: $from_t) -> Expr {
                    $crate::sym::Expr::$method(x)
                }
            }
        )*
    }
}

expand_conv_delegate! {
    impl From<u8> for Expr using integer;
    impl From<u64> for Expr using integer;
    impl From<i16> for Expr using integer;
    impl From<isize> for Expr using integer;
    impl From<i32> for Expr using integer;
    impl From<i64> for Expr using integer;
    impl From<u16> for Expr using integer;
    impl From<i8> for Expr using integer;
    impl From<usize> for Expr using integer;
    impl From<u32> for Expr using integer;
    impl From<BigUint> for Expr using integer;
    impl From<BigInt> for Expr using integer;

    impl From<f32> for Expr using approximate;
    impl From<f64> for Expr using approximate;
}

impl From<Symbol> for Expr {
    fn from(s: Symbol) -> Expr {
        Expr::Sym(s)
    }
}
