use super::Expr;
use super::Expr::*;
use super::Symbol;
use num::{BigInt, BigUint};
use std::ops::{Add, Mul, Div};

// impls for operators

// impl<E: Into<Expr>> Add<E> for Expr {
//     type Output = Expr;
//     fn add(self, rhs: E) -> Self::Output {
//         Sum(vec![self, rhs.into()])
//     }
// }
// 
// impl<E: Into<Expr>> Mul<E> for Expr {
//     type Output = Expr;
//     fn mul(self, rhs: E) -> Self::Output {
//         Product(vec![self, rhs.into()])
//     }
// }

impl<E: Into<Expr>> Div<E> for Expr {
    type Output = Expr;
    fn div(self, rhs: E) -> Self::Output {
        Ratio(Box::new(self), Box::new(rhs.into()))
    }
}

impl Add<Expr> for i32 {
    type Output = Expr;
    fn add(self, rhs: Expr) -> Self::Output {
        Sum(vec![self.into(), rhs])
    }
}

impl Div<Expr> for i32 {
    type Output = Expr;
    fn div(self, rhs: Expr) -> Self::Output {
        Ratio(Box::new(self.into()), Box::new(rhs))
    }
}

macro_rules! expand_op_impl {
    ($(impl<T: Into<Expr>> $trait:ident<T> for $lhs_t:ty { use $cons:ident as $op:ident; })* ) => {
        $(
            impl<T: Into<Expr>> $trait<T> for $lhs_t {
            type Output = Expr;
                fn $op(self, rhs: T) -> Self::Output {
                    $cons(vec![self.into(), rhs.into()])
                }
            }
        )*
    };
}

expand_op_impl! {
    impl<T: Into<Expr>> Add<T> for Expr { use Sum as add; }
    impl<T: Into<Expr>> Mul<T> for Expr { use Product as mul; }
    impl<T: Into<Expr>> Add<T> for Symbol { use Sum as add; }
    impl<T: Into<Expr>> Mul<T> for Symbol { use Product as mul; }
}

// impls for conversions

macro_rules! expand_conv_delegate {
    ( $(impl From<$from_t:ty> for Expr using $method:ident;)* ) => {
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
